use tokio::net::{TcpListener};
use tokio::io::{AsyncWriteExt, AsyncReadExt, BufReader, AsyncBufReadExt};

#[tokio::main]
async fn main() {
    // await keyword gets the result out of the future
    // To explore, remove .await from end of line and check
    // type of listener.
    let listener = TcpListener::bind("localhost:5000").await.unwrap();
    // Loop that enables multiple clients to connect
    loop {
        // Accept a connection
        let (mut socket, _addr) = listener.accept().await.unwrap();
            // Spawn a new task (think concept of multi-threading applied within a single thread)
            tokio::spawn(async move {
                // Split the read and write sides of the socket
                let (reader, mut writer) = socket.split();
                // Store for the data passed to the TCP Socket
                let mut reader = BufReader::new(reader);
                let mut line = String::new();
                loop {
                    // Read the bytes from the steam to the buffer and return
                    // the number of bytes read
                    let bytes_read = reader.read_line(&mut line).await.unwrap();
                    if bytes_read == 0 {
                        break;
                    }
                    // Echo the entire contents back to the client
                    // since the byte stream could be smaller than the size of the buffer,
                    // we will use write_all to truncate the space in the buffer that was unused

                    // Pass in the buffer as the source of data for the write back to the client
                    // through the socket, up to the number of bytes previously read by the socket.
                    writer.write_all(&mut line.as_bytes()).await.unwrap();
                    line.clear();
                }
            });
    }
}
