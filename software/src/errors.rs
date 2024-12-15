// use polars::prelude::PolarsError;
// use serde_json::Error;
use std::fmt;
use std::num::ParseIntError;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum MyError {
    EnvVarMissing(String),
    // We will defer to the parse error implementation for their error.
    // Supplying extra info requires adding more data to the type.
    MessageError(String),
    // Curl(curl::Error),
    OtherCurlError(String),
    PackageNotFoundError(String, String),
    FromUtf8Error,
    FmtError(std::fmt::Error),
    IoError(std::io::Error),
    ProcessError(String),
    ParseIntErr(ParseIntError),
    // TokioPostgres(tokio_postgres::Error),
    ChronoParseError(chrono::ParseError),
    MyCronParseError(String),
    // PolarsError(PolarsError),
    // TantivyError(Tantivy::Error)
    SerdeJsonError(serde_json::Error),
    HandebarsError(handlebars::TemplateError),
    RenderError(handlebars::RenderError),
}

// impl From<curl::Error> for MyError {
//     fn from(err: curl::Error) -> MyError {
//         MyError::Curl(err)
//     }
// }

impl From<FromUtf8Error> for MyError {
    fn from(_err: FromUtf8Error) -> MyError {
        MyError::FromUtf8Error
    }
}

impl From<std::fmt::Error> for MyError {
    fn from(err: std::fmt::Error) -> MyError {
        MyError::FmtError(err)
    }
}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> MyError {
        MyError::IoError(err)
    }
}
impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> MyError {
        MyError::ParseIntErr(err)
    }
}
impl From<serde_json::Error> for MyError {
    fn from(err: serde_json::Error) -> MyError {
        MyError::SerdeJsonError(err)
    }
}

// impl From<tokio_postgres::Error> for MyError {
//     fn from(err: tokio_postgres::Error) -> MyError {
//         MyError::TokioPostgres(err)
//     }
// }

impl From<chrono::ParseError> for MyError {
    fn from(err: chrono::ParseError) -> MyError {
        MyError::ChronoParseError(err)
    }
}

impl From<handlebars::TemplateError> for MyError {
    fn from(err: handlebars::TemplateError) -> MyError {
        MyError::HandebarsError(err)
    }
}
impl From<handlebars::RenderError> for MyError {
    fn from(err: handlebars::RenderError) -> MyError {
        MyError::RenderError(err)
    }
}

// impl From<PolarsError> for MyError {
//     fn from(err: PolarsError) -> MyError {
//         MyError::PolarsError(err)
//     }
// }

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::EnvVarMissing(name) => write!(f, "could not get env var {}", name),
            MyError::OtherCurlError(msg) => write!(f, "{}", msg),
            MyError::MessageError(msg) => write!(f, "{}", msg),
            // MyError::Curl(msg) => write!(f, "curl error {}", msg),
            MyError::FromUtf8Error => write!(f, "fromutf8error"),
            MyError::PackageNotFoundError(folder, product) => {
                write!(f, "could not find package {}/{}", folder, product)
            }
            MyError::FmtError(err) => {
                write!(f, "{}", err)
            }
            MyError::IoError(err) => {
                write!(f, "{}", err)
            }
            MyError::ProcessError(msg) => {
                write!(f, "process error {}", msg)
            }
            MyError::ParseIntErr(msg) => {
                write!(f, "parse int error {}", msg)
            }
            // MyError::TokioPostgres(msg) => {
            //     write!(f, "tokio postgres error {}", msg)
            // }
            MyError::ChronoParseError(msg) => {
                write!(f, "chrono parse error {}", msg)
            }
            MyError::MyCronParseError(name) => write!(f, "error for cron : {}", name),
            // MyError::PolarsError(err) => write!(f, "polors error : {}", err),
            MyError::SerdeJsonError(msg) => write!(f, "error for serde_json : {}", msg),
            MyError::HandebarsError(msg) => write!(f, "handlebars template error : {}", msg),
            MyError::RenderError(msg) => write!(f, "handlebars rendor error : {}", msg),
        }
    }
}
