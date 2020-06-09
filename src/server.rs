use std::io::prelude::*;
use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::serverconfig::Config;
use crate::threadpool::ThreadPool;

pub struct Server {
    config: Config,
}

impl Server{
    pub fn new() -> Server {
        Server {
            config: Config::get_config(),
        }
    }

    pub fn run(&self) -> Result<(), &'static str> {
        let config = &self.config;
        let listener;
        if let Ok(_listener) = TcpListener::bind(&config.url) {
            listener = _listener;
        } else {
            return Err("Could not bind to url. Check port privilege.");
        }

        let threadpool = ThreadPool::new(2);

        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                threadpool.run( || {
                    Server::handle_connection(stream);
                });
            }
        }

        Ok(())
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buf = [0; 512];
        stream.read(&mut buf).unwrap();

        // empty url
        let get = b"GET / HTTP/1.1\r\n";

        let mut response = String::new();
        
        if buf.starts_with(get) {
            if let Ok(html_content) = fs::read_to_string("index.html") {
                response = Server::return_ok(&html_content);
            }
        } else {
            response = Server::return_404();
        }

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn return_ok(content: &str) -> String {
        format!("HTTP/1.1 200 OK\r\n\r\n{}", content)
    }

    fn return_404() -> String {
        let mut response = String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n");
        if let Ok(error_content) = fs::read_to_string("404.html") {
            response.push_str(&format!("{}", error_content)[..]);
        }

        response
    }
}
