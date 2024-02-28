use std::net::TcpListener;
use std::io::Read;
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr: addr }
    }

    pub fn run(self) -> ! {
        print!("Listining on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {

            match listener.accept() {
                Ok((mut stream, _addr)) => {
                     let mut buffer = [0; 1024];
                     stream.read(&mut buffer);
                },
                Err(e) => println!("Faild to establish a connection: {}", e),

            }
        }
    }
}