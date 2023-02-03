use std::io::{Error, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::{Arc, LockResult, Mutex};
use log::{error, info, warn};
use std::env::temp_dir;


//todo: pub const QUEUE_PORT: u16 = 8080; //legacy support
pub const TRANSFER_PORT: u16 = 8082; //todo: argument

/* todo: actually wrap the data in a json object, adding two fields:
         - hashcode to ensure data was not corrupted (but not necessarily as we use tcp now)
         - kind of execution (now or queued), which needs further improvements for the renderer
 */


fn receive_file(stream: &mut TcpStream, prev_recv_count: u8) -> Result<PathBuf, Error> {
    let mut buffer: [u8; 4096] = [0; 4096];
    let mut file: Vec<u8> = Vec::new(); //u8 as in byte

    // Read all bytes until last byte is received
    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }
                file.extend_from_slice(&mut buffer[..bytes_read]);
            }
            Err(e) => {
                error!("Can't read from stream: {}", e.to_string());
                break;
            }
        };
    }

    // (Over)write the received file to disk if not empty
    if !file.is_empty() {
        let name = format!("received_file_{}.gif", prev_recv_count);
        let dir = temp_dir().join(name);

        return match std::fs::write(dir.clone(), file) {
            Ok(_) => {
                info!("Received file successfully! Written as ({})", dir.display());
                Ok(dir)
            }
            Err(e) => Err(e)
        };
    }
    Err(Error::new(std::io::ErrorKind::Other, "File is empty"))
}

pub fn start_recv_file_server(queue: Arc<Mutex<Vec<PathBuf>>>) {
    let mut prev_recv_count: u8 = 0;
    let ip = SocketAddr::from(([0, 0, 0, 0], TRANSFER_PORT));

    let listener = match TcpListener::bind(ip) {
        Ok(listener) => {
            info!("Open port {} for receiving animation over TCP", TRANSFER_PORT);

            for connection in listener.incoming() {
                let mut connection = connection.unwrap();
                match receive_file(&mut connection, prev_recv_count) {
                    Ok(path) => {
                        match queue.lock() {
                            Ok(mut vec) => {
                                let message = format!("Added ({}) to queue", path.display());
                                info!("{}", message);
                                match connection.write(message.as_ref()) {
                                    Ok(_) => {}
                                    Err(e) => {error!("Can't send conformation to host: {}", e.to_string())}
                                };
                                vec.push(path)
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