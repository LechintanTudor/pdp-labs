use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Response {
    status_line: String,
    headers: HashMap<String, String>,
    body: String,
}

impl Response {
    pub fn from_str(string: &str) -> Result<Response, ResponseParseError> {
        let mut line_iter = string.lines();

        let Some(status_line) = line_iter.next() else {
            return Err(ResponseParseError::InvalidStatusLine);
        };

        let headers = {
            let mut headers = HashMap::<String, String>::new();

            while let Some(line) = line_iter.next() {
                let line = line.trim();

                if line.is_empty() {
                    break;
                }

                let Some((key, value)) = line.split_once(':') else {
                    return Err(ResponseParseError::InvalidHeader);
                };

                headers.insert(key.to_string(), value.to_string());
            }

            headers
        };

        let body = {
            let mut body = line_iter.next().map(String::from).unwrap_or_default();

            for line in line_iter {
                body.push_str("\n");
                body.push_str(line);
            }

            body
        };

        Ok(Self { status_line: status_line.to_string(), headers, body })
    }

    #[inline]
    pub fn status_line(&self) -> &str {
        &self.status_line
    }

    pub fn headers(&self) -> impl Iterator<Item = (&str, &str)> {
        self.headers.iter().map(|(key, value)| (key.as_str(), value.as_str()))
    }

    #[inline]
    pub fn body(&self) -> &str {
        &self.body
    }
}

#[derive(Clone, Debug)]
pub enum ResponseParseError {
    InvalidStatusLine,
    InvalidHeader,
}

impl Error for ResponseParseError {}

impl fmt::Display for ResponseParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidStatusLine => write!(f, "Failed to parse response status line"),
            Self::InvalidHeader => write!(f, "Failed to parse response header"),
        }
    }
}
