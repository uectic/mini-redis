use std::{io::{BufRead, BufWriter, Write}, net::{TcpListener, TcpStream}};
use crate::{command, store, threadpool::ThreadPool};
use std::io::BufReader;
use std::sync::Mutex;
use std::sync::Arc;

pub fn run(socket: TcpListener){

    let db = Arc::new(Mutex::new(store::Db::new()));
    let thread_pool = ThreadPool::new(10);
    for stream in socket.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                println!("accept error: {e}");
                continue;
            }
        };
        let db = Arc::clone(&db);
        thread_pool.execute(move || {
            handle_con(stream, db);
        });
    }
}

fn handle_con(stream: TcpStream, db: Arc<Mutex<store::Db>>) {
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {}
            Err(e) => {
                println!("read error: {e}");
                break;
            }
        }
        let line = line.trim_end();
        if line.is_empty() {
            continue;
        }

        println!("{line}");

        let cmd = match command::Command::parse(line) {
            Ok(val) => val,
            Err(error) => {
                println!("{error}");
                continue;
            }
        };

        let res = {
            let mut db = db.lock().unwrap();
            db.run(cmd)
        };

        match res {
            Some(mut val) => {
                val.push('\n');
                if writer.write_all(val.as_bytes()).is_err() || writer.flush().is_err() {
                    println!("write failed");
                    break;
                }
            }
            None => println!("(nil)"),
        }
    }
}