use std::{io::BufRead, net::{TcpListener, TcpStream}};
use crate::{store, command};
use std::io::BufReader;

#[derive(Debug)]
struct Server{
    db: store::Db,
}

impl Server{
    fn new() -> Server{
        Server{
            db: store::Db::new(),
        }
    }

    fn get_db(&mut self) -> &mut store::Db{
        &mut self.db
    }
}

pub fn run(socket: TcpListener){

    let mut server = Server::new();
    for stream in socket.incoming(){
        let stream = stream.unwrap();
        handle_con(stream, &mut server);
    }
}

fn handle_con(stream: TcpStream, server: &mut Server){

    let mut reader = BufReader::new(&stream);
    for line in reader.lines(){
        let line = line.unwrap();
        println!("{}", line);

        let cmd = command::Command::parse(&line);
        let cmd = match cmd{
            Ok(val) => {
                val
            },
            Err(error) => {
                println!("{}", error.to_string());
                continue
            },
        };

        let res = server.get_db().run(&cmd);
        println!("{}", res.unwrap());

    }
}