// Copyright 2018 Gitai<i@gitai.me> All rights reserved.
//
// Permission is hereby granted, free of charge, to any person
// obtaining a copy of this software and associated documentation
// files (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify,
// merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall
// be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
// OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR
// ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF
// CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

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