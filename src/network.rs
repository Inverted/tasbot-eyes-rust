use std::env::temp_dir;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, LockResult, Mutex};

use clap::builder::Str;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::arguments::{ARGUMENTS, fallback_arguments};
use crate::network::PlayMode::{now, queued};

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
pub struct Message {
    mode: String,
    data: Vec<u8>,
}

pub struct ProcessedMessage {
    play_mode: PlayMode,
    path: PathBuf,
}

enum PlayMode {
    now,
    queued,
}

impl FromStr for PlayMode {
    type Err = NetworkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let binding = s.to_lowercase();
        let lower = binding.as_str();

        match lower {
            "now" => Ok(now),
            "queued" => Ok(queued),
            _ => Err(NetworkError::Conversion("Invalid")),
        }
    }
}

fn receive_file(stream: &mut TcpStream, prev_recv_count: u8) -> Result<ProcessedMessage, NetworkError> {
    let mut buffer: [u8; 4096] = [0; 4096];
    let mut data: Vec<u8> = Vec::new(); //u8 as in byte

    // Read all bytes until last byte is received
    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }
                data.extend_from_slice(&mut buffer[..bytes_read]);
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

pub fn start_recv_file_server(queue: Arc<Mutex<Vec<PathBuf>>>) {
    let binding = fallback_arguments();
    let args = ARGUMENTS.get().unwrap_or(&binding);

    let mut prev_recv_count: u8 = 0;
    let ip = SocketAddr::from(([0, 0, 0, 0], args.inject_port));

    let listener = match TcpListener::bind(ip) {
        Ok(listener) => {
            info!("Open port {} for receiving animation over TCP", ip.port());

            //Am I sorry for this tree? - No. Well, maybe.
            for connection in listener.incoming() {
                let mut connection = connection.unwrap();
                match receive_file(&mut connection, prev_recv_count) {
                    Ok(p_message) => {
                        match queue.lock() {
                            Ok(mut vec) => {
                                match p_message.play_mode {
                                    now => {
                                        todo!();
                                    }
                                    queued => {
                                        let message = format!("Added ({}) to queue", p_message.path.display());
                                        info!("{}", message);
                                        match connection.write(message.as_ref()) {
                                            Ok(_) => {}
                                            Err(e) => { error!("Can't send conformation to host: {}", e.to_string()) }
                                        };

                                        vec.push(p_message.path);
                                    }
                                }
                            }
                            Err(e) => error!("Can't access mutex: {}", e.to_string())
                        }
                    }
                    Err(e) => error!("Issue with received file: {}", e.to_string())
                };
                prev_recv_count += 1;
            }
        }
        Err(e) => {
            error!("Can't open port: {}", e.to_string());
        }
    };
}