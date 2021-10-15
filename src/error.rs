use std::path::PathBuf;

#[derive(Debug)]
pub enum Field {
    Forename,
    Surname,
    Email,
    Needle,
    Organisation,
    Telephone,
}

#[derive(Debug)]
pub enum SchemaFileProblem {
    NoComponents,
    WrongNumberOfComponents,
    InvalidUTF8,
    InvalidPath,
}

#[derive(Debug)]
pub enum Error {
    Empty(Field),
    Email(email_address::Error),
    Env,
    InvalidSchemaDirectory(PathBuf),
    InvalidSchemaFile(SchemaFileProblem),
    IO(std::io::Error),
    NoFile(PathBuf),
    NoSchemaFile(PathBuf),
    ParseInt(std::num::ParseIntError),
    Rusqlite(rusqlite::Error),
}

impl From<email_address::Error> for Error {
    fn from(e: email_address::Error) -> Self {
        Error::Email(e)
    }
}

impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Self {
        Error::Rusqlite(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IO(e)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::ParseInt(e)
    }
}
