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

use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::fs::DirEntry;
use std::path::Path;

trait Element {
    fn to_html(&self) -> String;
}

impl Element for DirEntry {
    fn to_html(&self) -> String {
        format!("<li><a href=\"{path}\">{filename}</a></li>",
                path = self.path().to_str().unwrap(),
                filename = self.file_name().to_str().unwrap())
    }
}

struct Index<'a> {
    path: &'a Path
}

impl<'a> Display for Index<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str("<ul>").unwrap();
        for p in self.path.read_dir().unwrap().map(|p| p.unwrap()) {
            f.write_fmt(format_args!("{}", p.to_html())).unwrap();
        }
        f.write_str("</ul>")
    }
}

struct Page<T> where T: Display {
    ele: T
}

impl<T> Display for Page<T> where T: Display {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str("<!DOCTYPE html>");
        f.write_str("<html>");
        f.write_fmt(format_args!("{}", self.ele));
        f.write_str("</html>")
    }
}

fn main() {
    let page: Page<Index> = Page {
        ele: Index {
            path: Path::new(".")
        }
    };
    println!("{}", page)
}