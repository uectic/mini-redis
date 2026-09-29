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

    pub fn parse(parts: &[String]) -> Result<Command, error::ParseError> {
        let Some(name) = parts.first() else {
            return Err(error::ParseError);
        };
        let args: Vec<&str> = parts.iter().map(String::as_str).collect();

        match name.to_ascii_lowercase().as_str() {
            "get"  => Get::parse_get(args).map(Command::Get),
            "set"  => Set::parse_set(args).map(Command::Set),
            "del"  => Del::parse_del(args).map(Command::Del),
            "ping" => Ping::parse_ping(args).map(Command::Ping),
            _  => Err(error::ParseError),
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

    pub fn parse_get(vec: Vec<&str>) -> Result<Get, error::ParseError>{
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

    pub fn into_parts(self) -> (String, String){
        (self.key, self.value)
    }

    pub fn parse_set(vec: Vec<&str>) -> Result<Set, error::ParseError>{
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

    pub fn parse_del(vec: Vec<&str>) -> Result<Del, error::ParseError>{
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

    pub fn msg(&self) -> &str{
        &self.msg
    }

    pub fn into_msg(self) -> String{
        self.msg
    }

    pub fn parse_ping(vec: Vec<&str>) -> Result<Ping, error::ParseError>{
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

