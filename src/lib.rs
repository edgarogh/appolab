use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::TcpStream;

const HOST: &str = "im2ag-appolab.u-ga.fr";
const PORT: u16 = 9999;

pub struct AppoLabConnection {
    tcp: TcpStream,
}

pub type WelcomeMessage = Box<[String]>;

impl AppoLabConnection {
    fn open() -> io::Result<(Self, WelcomeMessage)> {
        let mut stream = TcpStream::connect((HOST, PORT))?;

        let welcome_message = {
            let read = BufReader::new(&stream);
            read.lines()
                .take_while(|line| match line {
                    Ok(s) if s.is_empty() => false,
                    _ => true,
                })
                .collect::<Result<_, io::Error>>()?
        };

        stream.write_u32::<BigEndian>(0xFFFFFFCC)?;
        let _ignored = stream.read_u32::<BigEndian>()?;

        Ok((Self { tcp: stream }, welcome_message))
    }

    fn send_receive(&mut self, message: &str) -> io::Result<String> {
        self.tcp.write_u32::<BigEndian>(message.len() as u32)?;
        self.tcp.write_all(message.as_bytes())?;

        let resp = {
            let resp_len = self.tcp.read_u32::<BigEndian>()?;

            let mut buffer = vec![0u8; resp_len as usize];
            self.tcp.read_exact(buffer.as_mut())?;

            String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
        };

        Ok(resp)
    }

    fn close(self) {
        /* self drops */
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (username, password) = (
            std::env::var("APPOLAB_USERNAME").unwrap(),
            std::env::var("APPOLAB_PASSWORD").unwrap(),
        );

        let (mut conn, wms) = AppoLabConnection::open().unwrap();

        println!("{}", wms.join("\n"));

        let resp = conn
            .send_receive(&format!("login {} {}", password, username))
            .unwrap();

        println!("{}", resp);
    }
}
