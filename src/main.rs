use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}};


#[tokio::main]
async fn main() {
    // await keyword gets the result out of the future
    // To explore, remove .await from end of line and check
    // type of listener.
    let listener = TcpListener::bind("localhost:5000").await.unwrap();
    // Accept a connection
    let (mut socket, _addr) = listener.accept().await.unwrap();
    // Store for the data passed to the TCP Socket
    let mut buffer = [0u8; 1024];
    // Read the bytes from the steam to the buffer and return
    // the number of bytes read
    let bytes_read = socket.read(&mut buffer).await.unwrap();
    // Echo the entire contents back to the client
    // since the byte stream could be smaller than the size of the buffer,
    // we will use write_all to truncate the space in the buffer that was unused

    // Pass in the buffer as the source of data for the write back to the client
    // through the socket, up to the number of bytes previously read by the socket.
    socket.write_all(&buffer[..bytes_read]).await.unwrap();
}
