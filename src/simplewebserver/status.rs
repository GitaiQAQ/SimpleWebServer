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

//! HTTP status codes
use core::fmt;
use std::str::FromStr;

/// # Status Code Definitions
/// [[RFC1945, Section 9](https://tools.ietf.org/html/rfc1945#section-9)]
/// Each Status-Code is described below, including a description of which
/// method(s) it can follow and any meta information required in the
/// response.
#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    /// ## Informational
    /// [[RFC1945, Section 9.1](https://tools.ietf.org/html/rfc1945#section-9.1)]
    /// This class of status code indicates a provisional response,
    /// consisting only of the Status-Line and optional headers, and is
    /// terminated by an empty line. HTTP/1.0 does not define any 1xx status
    /// codes and they are not a valid response to a HTTP/1.0 request.
    /// However, they may be useful for experimental applications which are
    /// outside the scope of this specification.

    /// ## Successful
    /// [[RFC1945, Section 9.2](https://tools.ietf.org/html/rfc1945#section-9.2)]
    /// This class of status code indicates that the client's request was
    /// successfully received, understood, and accepted.
    ///
    /// 200 OK
    OK = 200,
    /// 201 Created
    Created = 201,
    /// 202 Accepted
    Accepted = 202,
    /// 204 No Content
    NoContent = 204,

    /// ## Redirection
    /// [[RFC1945, Section 9.3](https://tools.ietf.org/html/rfc1945#section-9.3)]
    /// This class of status code indicates that further action needs to be
    /// taken by the user agent in order to fulfill the request. The action
    /// required may be carried out by the user agent without interaction
    /// with the user if and only if the method used in the subsequent
    /// request is GET or HEAD. A user agent should never automatically
    /// redirect a request more than 5 times, since such redirections usually
    /// indicate an infinite loop.
    ///
    /// 300 Multiple Choices
    MultipleChoices = 300,
    /// 301 Move Permanently
    MovedPermanently = 301,
    /// 302 Move Temporarily
    MovedTemporarily = 302,
    /// 304 Not Modified
    NotModified = 304,

    /// ## Client Error 4xx
    /// [[RFC1945, Section 9.4](https://tools.ietf.org/html/rfc1945#section-9.4)]
    /// The 4xx class of status code is intended for cases in which the
    /// client seems to have erred. If the client has not completed the
    /// request when a 4xx code is received, it should immediately cease
    /// sending data to the server. Except when responding to a HEAD request,
    /// the server should include an entity containing an explanation of the
    /// error situation, and whether it is a temporary or permanent
    /// condition. These status codes are applicable to any request method.
    ///
    /// 400 Bad Request
    BadRequest = 400,
    /// 401 Unauthorized
    Unauthorized = 401,
    /// 403 Forbidden
    Forbidden = 403,
    /// 404 Not Found
    NotFound = 404,

    /// ## Server Error 5xx
    /// [[RFC1945, Section 9.5](https://tools.ietf.org/html/rfc1945#section-9.5)]
    /// Response status codes beginning with the digit "5" indicate cases in
    /// which the server is aware that it has erred or is incapable of
    /// performing the request. If the client has not completed the request
    /// when a 5xx code is received, it should immediately cease sending data
    /// to the server. Except when responding to a HEAD request, the server
    /// should include an entity containing an explanation of the error
    /// situation, and whether it is a temporary or permanent condition.
    /// These response codes are applicable to any request method and there
    /// are no required header fields.
    ///
    /// 500 Internal Server Error
    InternalServerError = 500,
    /// 501 Not Implemented
    NotImplemented = 501,
    /// 502 BadGateway
    BadGateway = 502,
    /// 503 Service Unavailable
    ServiceUnavailable = 503,
}

impl StatusCode {
    pub fn to_u16(&self) -> u16 {
        *self as u16
    }
}

impl FromStr for StatusCode {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, <Self as FromStr>::Err> {
        unimplemented!()
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match &self {
            StatusCode::OK => "OK",
            StatusCode::Created => "Created",
            StatusCode::Accepted => "Accepted",
            StatusCode::NoContent => "No Content",

            StatusCode::MultipleChoices => "Multiple Choices",
            StatusCode::MovedPermanently => "Moved Permanently",
            StatusCode::MovedTemporarily => "Moved Temporarily",
            StatusCode::NotModified => "Not Modified",

            StatusCode::BadRequest => "Bad Request",
            StatusCode::Unauthorized => "Unauthorized",
            StatusCode::Forbidden => "Forbidden",
            StatusCode::NotFound => "Not Found",

            StatusCode::InternalServerError => "Internal Server Error",
            StatusCode::NotImplemented => "Not Implemented",
            StatusCode::BadGateway => "Bad Gateway",
            StatusCode::ServiceUnavailable => "Service Unavailable",
        })
    }
}