use std::{io::BufWriter, net::{TcpListener, TcpStream}};
use crate::{command, store, threadpool::ThreadPool, parser};
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
        let parts = match parser::parse_resp(&mut reader) {
            Ok(Some(parts)) if !parts.is_empty() => parts,
            Ok(Some(_)) => continue,
            Ok(None) => break,
            Err(e) => {
                println!("resp error: {e}");
                let _ = parser::write_reply(&mut writer, &parser::Reply::Error(format!("ERR {e}")));
                break;
            }
        };

        let cmd = match command::Command::parse(&parts) {
            Ok(cmd) => cmd,
            Err(e) => {
                let _ = parser::write_reply(&mut writer, &parser::Reply::Error(format!("ERR {e}")));
                continue;
            }
        };

        let reply = {
            let mut db = db.lock().unwrap();
            db.run(cmd)
        };

        if parser::write_reply(&mut writer, &reply).is_err() {
            println!("write failed");
            break;
        }
    }
}