async fn client() -> Result<(), Error> {
  let stream = TcpStream::connect("127.0.0.1:8001").await?;
  let (mut reader, mut writer) = stream.into_split();
  tokio::spawn(async move {
    const BUFFERSIZE: usize = 256 * 1024;
    let mut buffer: [u8; BUFFERSIZE] = [0; BUFFERSIZE];
    loop {
      let len = reader.read(&mut buffer).await.unwrap();
      println!("Received {}", len);
      if len > 0 {
        println!(
          "TCP Client Received: {}",
          std::str::from_utf8(&buffer[0..len]).unwrap()
        );
      }
    }
  });
  //let _a = reader.read(&mut buf).await.unwrap();
  let stdin = io::stdin();
  for line in stdin.lock().lines() {
    let text = line.unwrap();
    println!("Send {} To TCP Server", text);
    let vec = text.as_bytes();
    writer.write(vec).await.unwrap();
  }
  Ok(())
}
