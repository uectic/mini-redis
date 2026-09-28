use crate::error;

#[derive(Debug)]
pub enum Command{
    Get(Get),
    Set(Set),
    Del(Del),
    Ping(Ping),
    Unknown(Unknown),
}

impl Command{

    pub fn parse(cmd: &str) -> Result<Command, error::ParseError>{
        let cmd_vec: Vec<&str> = cmd.split(' ').collect();
        match cmd_vec[0]{
            "get" => {
                match Get::parse_get(&cmd_vec){
                    Ok(val) => {
                        Ok(Command::Get(val))
                    },
                    Err(error) => {
                        Err(error)
                    },
                }
            },
            "set" =>{
                match Set::parse_set(&cmd_vec){
                    Ok(val) => {
                        Ok(Command::Set(val))
                    },
                    Err(error) => {
                        Err(error)
                    },
                }
            },
            "del" => {
                match Del::parse_del(&cmd_vec){
                    Ok(val) => {
                        Ok(Command::Del(val))
                    },
                    Err(error) => {
                        Err(error)
                    },
                }
            },
            "ping" => {
                match Ping::parse_ping(&cmd_vec){
                    Ok(val) => {
                        Ok(Command::Ping(val))
                    },
                    Err(error) => {
                        Err(error)
                    },
                }
            },
            _ => {
                match Unknown::parse_unknown(){
                    Ok(val) => {
                        Ok(Command::Unknown(val))
                    },
                    Err(error) => {
                        Err(error)
                    },
                }
            },
        }
    }
}

#[derive(Debug)]
pub struct Get{
    key: String,
}

impl Get{
    pub fn new<F>(key: F) -> Get
        where F: ToString
    {
        Get { key: key.to_string() }
    }

    pub fn key(&self) -> &str{
        &self.key
    }

    pub fn parse_get(vec: &[&str]) -> Result<Get, error::ParseError>{
        if vec.len()!=2{
            Err(error::ParseError)
        } else{
            Ok(Get::new(vec[1]))
        }
    }
}

#[derive(Debug)]
pub struct Set{
    key: String,
    value: String,
}

impl Set{
    pub fn new<F>(key: F, value: F) -> Set
        where F: ToString
    {
        Set { key: key.to_string(), value: value.to_string() }
    }

    pub fn key(&self) -> &str{
        &self.key
    }

    pub fn value(&self) -> &str{
        &self.value
    }

    pub fn parse_set(vec: &[&str]) -> Result<Set, error::ParseError>{
        if vec.len()!=3{
            Err(error::ParseError)
        } else{
            Ok(Set::new(vec[1], vec[2]))
        }
    }
}

#[derive(Debug)]
pub struct Del{
    key : String,
}

impl Del{
    pub fn new<F>(key: F) -> Del
        where F: ToString
    {
        Del { key: key.to_string() }
    }

    pub fn key(&self) -> &str{
        &self.key
    }

    pub fn parse_del(vec: &[&str]) -> Result<Del, error::ParseError>{
        if vec.len()!=2{
            Err(error::ParseError)
        } else{
            Ok(Del::new(vec[1]))
        }
    }
}

#[derive(Debug)]
pub struct Ping{
    msg: String,
}

impl Ping{
    pub fn new<F>(key: Option<F>) -> Ping
        where F: ToString
    {
        match key{
            Some(msg)=>{
                Ping{
                    msg: msg.to_string(),
                }
            },
            None =>{
                Ping {
                    msg: String::from(""),
                }
            }
        }
    }

    pub fn key(&self) -> &str{
        &self.msg
    }

    pub fn parse_ping(vec: &[&str]) -> Result<Ping, error::ParseError>{
        if vec.len()==1{
            Ok(Ping::new(Some(String::from(""))))
        } 
        else if vec.len() == 2{
            Ok(Ping::new(Some(vec[1])))
        } 
        else{
            Err(error::ParseError)
        }
    }
}

#[derive(Debug)]
pub struct Unknown{
}

impl Unknown{
    pub fn parse_unknown() -> Result<Unknown, error::ParseError>{
        Err(error::ParseError)
    }
}

