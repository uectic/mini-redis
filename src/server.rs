use std::{io::{BufRead, BufWriter, Write}, net::{TcpListener, TcpStream}};
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
    let mut writer = BufWriter::new(&stream);
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

        let res = server.get_db().run(cmd);
        match res {
            Some(mut val) => {
                val.push('\n');
                match writer.write_all(val.as_bytes()){
                    Err(_) =>{
                        println!("write failed")
                    },
                    Ok(_)=>{
                        if let Ok(_) = writer.flush(){
                            print!("{}", val)
                        } else{
                            println!("write failed")
                        }
                    }
                }
            }
            None => println!("(nil)"),
        }
    }
}