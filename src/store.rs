use std::collections::HashMap;
use crate::command::{self, Unknown};

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

    pub fn run<'a>(&'a mut self, cmd: &'a command::Command) -> Option<&'a str>{
        match cmd{
            command::Command::Get(get) => {
                let res = self.mem.get(get.key())?;
                Some(res)
            },
            command::Command::Set(set) => {
                self.mem.insert(set.key().to_string(), set.value().to_string())?;
                Some("Success")
            },
            command::Command::Del(del) => {
                self.mem.remove(del.key())?;
                Some("Success")
            }
            command::Command::Ping(ping) => {
                if ping.msg() == ""{
                    Some("PONG")
                } else{
                    let res = ping.msg();
                    Some(res)
                }
            }
            command::Command::Unknown(unknown) => {
                Some("Failure")
            },
        }
    }
}