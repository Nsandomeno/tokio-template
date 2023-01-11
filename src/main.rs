use tokio::net::{TcpListener};
use tokio::sync::{broadcast};
use tokio::io::{AsyncWriteExt, BufReader, AsyncBufReadExt};

const MAX_BROADCAST_CAPACITY: usize = 10;

#[tokio::main]
async fn main() {
    // await keyword gets the result out of the future
    // To explore, remove .await from end of line and check
    // type of listener.
    let listener = TcpListener::bind("localhost:5000").await.unwrap();
    // Broadcast channel for chat server - multiple producers/consumers on a single channel
    // In our case this means we have a sender/receiver for every async task
    let (tx, _rx) = broadcast::channel(MAX_BROADCAST_CAPACITY);
    // Loop that enables multiple clients to connect
    loop {
        // Accept a connection
        let (mut socket, addr) = listener.accept().await.unwrap();
            // Clone the sender of the broadcast channel so that it can be moved into the async task
            let tx = tx.clone();
            // Subscribe to the broadcast sender with a receiver of the broadcast channel. Done so
            // that it too can be moved into the async task
            let mut rx = tx.subscribe();
            // Spawn a new task (think concept of multi-threading applied within a single thread)
            tokio::spawn(async move {
                // Split the read and write sides of the socket
                let (reader, mut writer) = socket.split();
                // Store for the data passed to the TCP Socket
                let mut reader = BufReader::new(reader);
                let mut line = String::new();
                loop {
                    // A common pattern is a select statement with 5 or 6 branches on it
                    // that are all just gathering messages from different channels
                    // and using some shared state.
                    tokio::select! {
                        // Either "Send to" the broadcast channel end in the async task...
                        result = reader.read_line(&mut line) => {
                            // Read the bytes from the steam to the buffer and return
                            // the number of bytes read
                            if result.unwrap() == 0 {
                                break;
                            }
                            // Sending the read line from the async task (sinc the sender of the broadcast channel was moved into the block) 
                            tx.send((line.clone(), addr)).unwrap();
                            line.clear();
                        }
                        // ... Or, "Receive from" the broadcast channel end in the async task
                        result = rx.recv() => {
                            let (msg, sender_addr) = result.unwrap();
                            // Echo the entire contents back to the client
                            // since the byte stream could be smaller than the size of the buffer,
                            // we will use write_all to truncate the space in the buffer that was unused

                            // Pass in the buffer as the source of data for the write back to the client
                            // through the socket, up to the number of bytes previously read by the socket.
                            if addr != sender_addr { // If message is coming from another client
                                writer.write_all(&mut msg.as_bytes()).await.unwrap();
                            }
                        }
                    }
                }
            });
    }
}
