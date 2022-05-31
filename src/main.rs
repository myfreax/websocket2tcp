use futures_util::{SinkExt, StreamExt};
use log::info;
use std::io::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let _ = env_logger::try_init();
  let tcp_server_addr = "192.168.1.6:5555";
  let websocket_server_addr = "127.0.0.1:8080";
  // Connect Tcp Server
  let tcp_client = TcpStream::connect(tcp_server_addr).await?;
  let (mut tcp_reader, mut tcp_writer) = tcp_client.into_split();

  // Create Websocket Server
  let ws_listener = TcpListener::bind(&websocket_server_addr).await?;
  info!("Listening on: {}", websocket_server_addr);

  // Only Accept One Websocket Client Connection
  let (ws_server, _) = ws_listener.accept().await.unwrap();
  let addr = ws_server.peer_addr().unwrap();
  info!("Accepted Client Connection {}", addr);
  let ws_stream = tokio_tungstenite::accept_async(ws_server).await.unwrap();
  let (mut ws_writer, mut ws_reader) = ws_stream.split();

  // Read Tcp Client Data and Send To Browser by Websocket Server
  tokio::spawn(async move {
    const BUFFERSIZE: usize = 256 * 1024;
    let mut buffer: [u8; BUFFERSIZE] = [0; BUFFERSIZE];
    loop {
      let len = tcp_reader.read(&mut buffer).await.unwrap();
      if len > 0 {
        let recv_buffer = &buffer[0..len];
        let recv_vec: Vec<u8> = recv_buffer.to_vec();
        let msg = Message::Binary(recv_vec);
        ws_writer.send(msg).await.unwrap();
        info!(
          "TCP Client Received: {}",
          std::str::from_utf8(&buffer[0..len]).unwrap()
        );
      }
    }
  });

  //Read Websocket Server Data and Send Tcp Server By Tcp Client
  while let Some(msg) = ws_reader.next().await {
    let msg = msg.unwrap();
    let buffer: &[u8] = &msg.into_data();
    tcp_writer.write(buffer).await.unwrap();
  }
  Ok(())
}
