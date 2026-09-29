use std::collections::HashMap;
use crate::command::{self};
use crate::parser;

#[derive(Debug)]
pub struct Db{
    
    mem: HashMap<String, String>,
}

impl Db{
    pub fn new() -> Db{
        Db{
            mem: HashMap::new(),
        }
    }

    pub fn run(&mut self, cmd: command::Command) -> parser::Reply {
        match cmd {
            command::Command::Get(get) => match self.mem.get(get.key()) {
                Some(v) => parser::Reply::Bulk(v.clone()),
                None => parser::Reply::Null,
            },
            command::Command::Set(set) => {
                let (key, value) = set.into_parts();
                self.mem.insert(key, value);
                parser::Reply::String("OK".to_string())
            }
            command::Command::Del(del) => {
                let removed = self.mem.remove(del.key()).is_some();
                parser::Reply::Integer(if removed { 1 } else { 0 })
            }
            command::Command::Ping(ping) =>{
                if ping.msg() == ""{
                    parser::Reply::String("PONG".to_string())
                } else{
                    parser::Reply::Bulk(ping.into_msg())
                }
            },
            command::Command::Unknown(_) =>{
                parser::Reply::Error("unknown command".to_string())
            }
        }
    }
}