use bytes::BytesMut;
use futures::SinkExt;
use std::{error::Error, fmt, io};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixListener;
use tokio_util::codec::{Decoder, Encoder, Framed};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = UnixListener::bind("/tmp/tokio.domain.socket")?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}

struct Protocol;

impl Decoder for Protocol {
    type Item = Vec<u8>;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> io::Result<Option<Vec<u8>>> {
        Ok(Some(vec![]))
    }
}

impl Encoder<Vec<u8>> for Protocol {
    type Error = io::Error;

    fn encode(&mut self, item: Vec<u8>, dst: &mut BytesMut) -> io::Result<()> {
        Ok(())
    }
}
