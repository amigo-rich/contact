use std::path::PathBuf;

pub enum Field {
    Forename,
    Surname,
    Email,
    Needle,
    Organisation,
    Telephone,
}

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let what = match self {
            Field::Forename => "forename",
            Field::Surname => "surname",
            Field::Email => "email",
            Field::Needle => "needle",
            Field::Organisation => "organisation",
            Field::Telephone => "telephone",
        };
        write!(f, "Field {} is empty.", what)
    }
}

pub enum Error {
    Empty(Field),
    Email(email_address::Error),
    Env,
    NoArg,
    NoFile(PathBuf),
    Rusqlite(rusqlite::Error),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Empty(t) => write!(f, "{:?}", t),
            Error::Email(e) => write!(f, "Parsing email address failed: '{}'", e),
            Error::Env => write!(f, "Environemnt variable CONTACT_DB is not set"),
            Error::NoArg => write!(f, "Usage: 'contact add' or 'contact search'"),
            Error::NoFile(p) => write!(f, "The requested file was not found: '{:?}'", p),
            Error::Rusqlite(e) => write!(f, "A rusqlite error occurred: ;{}", e),
        }
    }
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
