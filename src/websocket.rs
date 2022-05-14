async fn accept_connection(stream: TcpStream) {
  let addr = stream.peer_addr().unwrap();
  info!("Accepted Client Connection {}", addr);
  let ws_stream = tokio_tungstenite::accept_async(stream).await.unwrap();
  let (write, read) = ws_stream.split();
  // We should not forward messages other than text or binary.
  read
    .try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
    .forward(write)
    .await
    .expect("Failed to forward messages")
}

async fn start() -> Result<(), Error> {
  let addr = "127.0.0.1:8080";
  let listener = TcpListener::bind(&addr).await.unwrap();
  info!("Listening on: {}", addr);
  while let Ok((stream, _)) = listener.accept().await {
    tokio::spawn(accept_connection(stream));
  }
  Ok(())
}
