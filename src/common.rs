use core::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct HTTPVersion {
    major: u8,
    minor: u8,
}

impl HTTPVersion {
    pub fn new(major: u8, minor: u8) -> Self {
        HTTPVersion {
            major,
            minor,
        }
    }
}

impl Default for HTTPVersion {
    fn default() -> Self {
        HTTPVersion {
            major: 0,
            minor: 0,
        }
    }
}

impl fmt::Display for HTTPVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HTTP/{}.{}", self.major, self.minor)
    }
}

impl FromStr for HTTPVersion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut result = s.split(|c|
            c == '/' ||
                c == '.').filter(|k| !k.is_empty());
        result.next(); // HTTP
        Ok(HTTPVersion {
            major: u8::from_str(result.next().unwrap()).unwrap(),
            minor: u8::from_str(result.next().unwrap()).unwrap(),
        })
    }
}