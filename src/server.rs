use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr: addr }
    }

    //deallocate the sever, if self without &
    //let it take the ownership as we dont use it anymore
    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut tcpstream, socketaddr)) => {
                    let mut buffer = [0; 1024];
                    match tcpstream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {:?}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {}
                                Err(e) => println!("Faile to convert {}", e),
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("failed to establish a connection: {}", e),
            }
        }

        // let tup = (5, "a", listener);
    }
}
