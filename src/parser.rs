use std::io::BufRead;
use std::io::self;
use std::io::Read;
use std::io::Write;

#[derive(Debug)]
pub enum RespError {
    Io(io::Error),
    Protocol(String),
}

impl std::fmt::Display for RespError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RespError::Io(e) => write!(f, "io error: {e}"),
            RespError::Protocol(msg) => write!(f, "protocol error: {msg}"),
        }
    }
}

impl std::error::Error for RespError {}

impl From<io::Error> for RespError {
    fn from(e: io::Error) -> Self {
        RespError::Io(e)
    }
}


#[derive(Debug, PartialEq)]
pub enum Reply{

    Null,
    NullArray,
    String(String),
    Error(String),
    Integer(i64),
    Bulk(String),
    BufBulk(Vec<u8>),
    Array(Vec<Reply>)
}

pub fn write_reply(writer: &mut impl Write, reply: &Reply) -> io::Result<()> {
    match reply {
        Reply::String(s) => write!(writer, "+{s}\r\n"),
        Reply::Error(s) => write!(writer, "-{s}\r\n"),
        Reply::Integer(n) => write!(writer, ":{n}\r\n"),
        Reply::Bulk(s) => write!(writer, "${}\r\n{}\r\n", s.len(), s),
        Reply::BufBulk(s) => {
            write!(writer, "${}\r\n", s.len())?;
            writer.write_all(&s)?;
            writer.write_all(b"\r\n")
        },
        Reply:: Array(s) => {
            write!(writer, "*{}\r\n", s.len())?;
            for item in s {
                write_reply(writer, item)?;
            }
            Ok(())
        }
        Reply::Null => write!(writer, "$-1\r\n"),
        Reply::NullArray => write!(writer, "*-1\r\n"),
    }?;
    writer.flush()
}


pub fn parse_resp(reader: &mut impl BufRead) -> Result<Option<Vec<String>>, RespError>{
    
    let mut line = String::new();
    let bytes_read = reader.read_line(&mut line)?;
    if bytes_read == 0{
        return Ok(None);
    }

    let line = trim_crlf(&line);

    if !line.starts_with('*'){
        return Err(RespError::Protocol(format!("expected '*', got: {line:?}")));
    }

    let count: i64= line[1..].parse()
        .map_err(|_| RespError::Protocol(format!("bad array length: {line:?}")))?;

    if count<=0{
        return Ok(Some(Vec::new()));
    }
    let mut parts = Vec::with_capacity(count as usize);
    for _ in 1..count{
        parts.push(read_bulk_string(reader)?);
    }
    
    return Ok(Some(parts));
}


fn read_bulk_string(reader: &mut impl BufRead) -> Result<String, RespError>{

    let mut line = String::new();
    reader.read_line(&mut line)?;
    let line = trim_crlf(&line);

    if !line.starts_with('$'){
        return Err(RespError::Protocol(format!("expected '$', got: {line:?}")));
    }

    let count: i64 = line.parse()
        .map_err(|_|RespError::Protocol(format!("bad bulk length: {line:?}")))?;

    if count<=0{
        return Ok(String::new());
    }

    let len = count as usize;
    let mut buf = vec![0u8; len+2];
    reader.read_exact(&mut buf)?;
    buf.truncate(len);

    String::from_utf8(buf).map_err(|_| RespError::Protocol("non-utf8 bulk string".to_string()))

}

fn trim_crlf(line: &str) -> &str{
    line.trim_end_matches('\n').trim_end_matches('\r')
}