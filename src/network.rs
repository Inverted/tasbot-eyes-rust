use std::env::temp_dir;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use log::{error, info};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::arguments::{ARGUMENTS, fallback_arguments};
use crate::network::PlayMode::{Now, Queued};

//todo: pub const QUEUE_PORT: u16 = 8080; //legacy support

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("An IO error occurred: {0}")]
    Io(#[from] std::io::Error),

    #[error("An JSON error occurred: {0}")]
    JSON(#[from] serde_json::Error),

    #[error("An conversion error occurred: {0}")]
    Conversion(&'static str),
}

#[derive(Serialize, Deserialize, Debug)]
/// Structure the received JSON gets parsed to
pub struct Message {
    /// Mode how the animation should be played
    mode: String,

    /// A list of bytes of the animation
    data: Vec<u8>,
}

/// Once the `Message` is interpret, present it as a `ProcessedMessage`
pub struct ProcessedMessage {
    /// The `PlayMode` how the animation should be played
    play_mode: PlayMode,

    /// `PathBuf` to the animation
    path: PathBuf,
}

/// How the animation should be played
enum PlayMode {

    ///Play the animation immediately
    Now,

    /// Queue the animation as next to play after the current blink cycle
    Queued,
}

impl FromStr for PlayMode {
    type Err = NetworkError;

    /// Convert the received play mode `String` to an enum variant
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        match lower.as_str() {
            "now" => Ok(Now),
            "queued" => Ok(Queued),
            _ => Err(NetworkError::Conversion("Invalid play mode type")),
        }
    }
}

/// Open a TCP port and start receiving messages. Likely started as thread.
pub fn start_recv_file_server(queue: Arc<Mutex<Vec<PathBuf>>>) {
    let binding = fallback_arguments();
    let args = ARGUMENTS.get().unwrap_or(&binding);

    let mut prev_recv_count: u8 = 0;
    let ip = SocketAddr::from(([0, 0, 0, 0], args.inject_port));

    /* Am I sorry for this tree? - No. Well, maybe. */

    // Open port and bind
    match TcpListener::bind(ip) {
        Ok(listener) => {
            info!("Open port {} for receiving animation over TCP", ip.port());

            //Wait for incoming connections
            for connection in listener.incoming() {
                let mut connection = connection.unwrap();

                //Once a connection is established, start receiving
                match receive_message(&mut connection, prev_recv_count) {
                    Ok(p_message) => {

                        //When message is received, check its play mode
                        match p_message.play_mode {

                            //Play immediately
                            Now => {
                                todo!();

                                let message = format!("Play ({}) now!", p_message.path.display());
                                send_answer(&mut connection, message);
                            }

                            //Queue animation
                            Queued => {

                                //Acquire lock
                                match queue.lock() {
                                    Ok(mut vec) => {
                                        //Send answer
                                        let message = format!("Add ({}) to queue", p_message.path.display());
                                        send_answer(&mut connection, message);

                                        //Add to queue
                                        vec.push(p_message.path);
                                    }
                                    Err(e) => error!("Can't access mutex: {}", e.to_string())
                                }
                            }
                        }
                    }
                    Err(e) => error!("Issue with received file: {}", e.to_string())
                };

                //Increase counter to have incrementing file names
                prev_recv_count += 1;
            }
        }
        Err(e) => {
            error!("Can't open port: {}", e.to_string());
        }
    };
}

/// Send an answer to a connection
///
/// # Input
/// * `connection`: A `TcpStream` the answer should be send to
/// * `message`: The answer, that shoudl be send
fn send_answer(connection: &mut TcpStream, answer: String) {
    info!("{}", answer);
    match connection.write(answer.as_ref()) {
        Ok(_) => {}
        Err(e) => { error!("Can't send conformation to host: {}", e.to_string()) }
    };
}

/// Receive a message from a stream
///
/// # Input
/// * `connection`: A `TcpStream`, that should be listend to
/// * `prev_recv_count`: Indicates how many messages have been received before
///
/// # Output
/// A `Result<ProcessedMessage, NetworkError>` with
/// * a `ProcessedMessage`, containing the `PlayMode` of the animation and a `PathBuf` where the animation is stored
/// * a `NetworkError` is thrown, when:
///     - `process_message()` fails
///     - the received message is empty
fn receive_message(connection: &mut TcpStream, prev_recv_count: u8) -> Result<ProcessedMessage, NetworkError> {
    let mut buffer: [u8; 4096] = [0; 4096];
    let mut data: Vec<u8> = Vec::new(); //u8 as in byte

    // Read all bytes until last byte is received
    loop {
        match connection.read(&mut buffer) {
            Ok(bytes) => {
                if bytes == 0 {
                    break;
                }
                data.extend_from_slice(&mut buffer[..bytes]);
            }
            Err(e) => {
                error!("Can't read from stream: {}", e.to_string());
                break;
            }
        };
    }

    // (Over)write the received file to disk if not empty
    if !data.is_empty() {
        match String::from_utf8(data) {
            Ok(json) => {
                let message: Message = serde_json::from_str(&json)?;
                return process_message(message, prev_recv_count);
            }
            Err(e) => error!("Can't read received JSON object: {}", e.to_string()),
        }
    }
    Err(NetworkError::from(std::io::Error::new(std::io::ErrorKind::Other, "File is empty")))
}

/// Process a received message by writing the animation to an file and determine the play mode
///
/// # Input
/// * `message`: The received message string
/// * `prev_recv_count`: Indicates how many messages have been received before
///
/// # Output
/// A `Result<ProcessedMessage, NetworkError>` with
/// * a `ProcessedMessage`, containing the `PlayMode` of the animation and a `PathBuf` where the animation is stored
/// * a `NetworkError` is thrown, when:
///     - `process_message()` fails
///     - the received message is empty
fn process_message(message: Message, prev_recv_count: u8) -> Result<ProcessedMessage, NetworkError> {
    let name = format!("received_file_{}.gif", prev_recv_count);
    let dir = temp_dir().join(name);

    return match std::fs::write(dir.clone(), message.data) {
        Ok(_) => {
            info!("Received file successfully! Written as ({}). Play mode is {}", dir.display(), message.mode);

            Ok(ProcessedMessage {
                play_mode: PlayMode::from_str(&*message.mode)?,
                path: dir,
            })
        }
        Err(e) => Err(NetworkError::from(e))
    };
}