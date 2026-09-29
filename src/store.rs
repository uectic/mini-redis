use std::collections::HashMap;
use crate::command::{self};

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

    pub fn run(&mut self, cmd: command::Command) -> Option<String>{
        match cmd{
            command::Command::Get(get) => {
                self.mem.get(get.key()).cloned()
            },
            command::Command::Set(set) => {
                self.mem.insert(set.key().to_string(), set.value().to_string());
                Some("OK".to_string())
            },
            command::Command::Del(del) => {
                let removed = self.mem.remove(del.key()).is_some();
                Some(if removed { "1".to_string() } else { "0".to_string() })
            }
            command::Command::Ping(ping) => {
                if ping.msg().is_empty() {
                    Some("PONG".to_string())
                } else {
                    Some(ping.into_msg())
                }
            }
            command::Command::Unknown(_) => {
                Some("ERR unknown command".to_string())
            },
        }
    }
}