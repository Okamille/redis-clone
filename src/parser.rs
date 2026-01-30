use std::io::{self, BufRead, Read};

#[derive(Debug, PartialEq, Clone)]
pub enum RespMessage {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Vec<u8>),
    Array(Vec<RespMessage>),
    Null,
}

pub struct RespParser<R> {
    reader: io::BufReader<R>,
}

impl<R: Read> RespParser<R> {
    pub fn new(inner: R) -> Self {
        return RespParser {
            reader: io::BufReader::new(inner),
        };
    }

    pub fn parse_next(&mut self) -> io::Result<RespMessage> {
        let mut prefix = [0u8];
        self.reader.read_exact(&mut prefix)?;

        match prefix[0] {
            b'+' => self.parse_simple_string(),
            b':' => self.parse_integer(),
            b'$' => self.parse_bulk_string(),
            b'*' => self.parse_array(),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown prefix")),
        }
    }

    pub fn parse_simple_string(&mut self) -> io::Result<RespMessage> {
        let content = self.read_line()?;
        Ok(RespMessage::SimpleString(content.to_string()))
    }

    pub fn parse_integer(&mut self) -> io::Result<RespMessage> {
        let line = self.read_line()?;
        let n = line
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid integer format"))?;

        Ok(RespMessage::Integer(n))
    }

    pub fn parse_bulk_string(&mut self) -> io::Result<RespMessage> {
        let line = self.read_line()?;
        let length: i64 = line.parse().map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Invalid bulk string length")
        })?;

        if length == -1 {
            return Ok(RespMessage::Null);
        }

        let mut buffer = vec![0u8; length as usize];
        self.reader.read_exact(&mut buffer)?;

        let mut crlf = [0u8; 2];
        self.reader.read_exact(&mut crlf)?;

        Ok(RespMessage::BulkString(buffer))
    }

    pub fn parse_array(&mut self) -> io::Result<RespMessage> {
        let line = self.read_line()?;
        let length: i64 = line
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid array length"))?;

        let mut array = Vec::with_capacity(length as usize);
        for _ in 0..length {
            array.push(self.parse_next()?);
        }

        Ok(RespMessage::Array(array))
    }

    fn read_line(&mut self) -> io::Result<String> {
        let mut line = String::new();
        self.reader.read_line(&mut line)?;

        if line.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Connection closed",
            ));
        }

        Ok(line.trim_end_matches("\r\n").to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::parser::{RespMessage, RespParser};

    #[test]
    fn test_parse_simple_string() {
        let data = b"+OK\r\n";
        let mut parser = RespParser::new(Cursor::new(data));
        let result = parser.parse_next().unwrap();

        assert_eq!(result, RespMessage::SimpleString("OK".to_string()));
    }

    #[test]
    fn test_parse_integer() {
        let data = b":1000\r\n";
        let mut parser = RespParser::new(Cursor::new(data));
        let result = parser.parse_next().unwrap();

        assert_eq!(result, RespMessage::Integer(1000));
    }

    #[test]
    fn test_parse_bulk_string() {
        let data = b"$5\r\nhello\r\n";
        let mut parser = RespParser::new(Cursor::new(data));
        let result = parser.parse_next().unwrap();

        assert_eq!(result, RespMessage::BulkString(b"hello".to_vec()));
    }

    #[test]
    fn test_parse_array() {
        let data = b"*3\r\n$5\r\nhello\r\n:1000\r\n+OK\r\n";
        let mut parser = RespParser::new(Cursor::new(data));
        let result = parser.parse_next().unwrap();

        assert_eq!(
            result,
            RespMessage::Array(vec![
                RespMessage::BulkString(b"hello".to_vec()),
                RespMessage::Integer(1000),
                RespMessage::SimpleString("OK".to_string())
            ])
        );
    }
}
