use std::{io::BufRead, net::{TcpListener, TcpStream}};
use crate::{store};
use std::io::BufReader;

#[derive(Debug)]
struct Server{
    socket: TcpListener,
    db: store::Db,
}

impl Server{
    fn new(socket: TcpListener) -> Server{
        Server{
            socket,
            db: store::Db::new(),
        }
    }

    fn get_socket(&self) -> &TcpListener{
        &self.socket
    }
}

pub fn run(socket: TcpListener){

    let server = Server::new(socket);
    for stream in server.get_socket().incoming(){
        let stream = stream.unwrap();
        handle_con(stream);
    }
}

pub fn handle_con(stream: TcpStream){

    let mut reader = BufReader::new(stream);
    for line in reader.lines(){
        let line = line.unwrap();
        println!("{}", line);
    }
}