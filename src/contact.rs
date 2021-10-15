use crate::error::{Error, Field};
use email_address::EmailAddress;
use std::str::FromStr;

#[derive(Debug)]
pub struct Contact {
    forename: String,
    surname: String,
    email: EmailAddress,
    organisation: Option<String>,
    telephone: Option<String>,
}

impl Contact {
    pub fn new(forename: &str, surname: &str, email: &str) -> Result<Self, Error> {
        if forename.is_empty() {
            return Err(Error::Empty(Field::Forename));
        } else if surname.is_empty() {
            return Err(Error::Empty(Field::Surname));
        } else if email.is_empty() {
            return Err(Error::Empty(Field::Email));
        }
        let email = EmailAddress::from_str(email)?;
        Ok(Contact {
            forename: forename.to_string(),
            surname: surname.to_string(),
            email,
            organisation: None,
            telephone: None,
        })
    }
    pub fn set_organisation(&mut self, organisation: &str) -> Result<(), Error> {
        if organisation.is_empty() {
            return Err(Error::Empty(Field::Organisation));
        }
        self.organisation = Some(organisation.to_string());
        Ok(())
    }
    pub fn set_telephone(&mut self, telephone: &str) -> Result<(), Error> {
        if telephone.is_empty() {
            return Err(Error::Empty(Field::Telephone));
        }
        self.telephone = Some(telephone.to_string());
        Ok(())
    }
    pub fn forename(&self) -> &str {
        &self.forename
    }
    pub fn surname(&self) -> &str {
        &self.surname
    }
    pub fn email(&self) -> &EmailAddress {
        &self.email
    }
    pub fn organisation(&self) -> Option<&str> {
        self.organisation.as_deref()
    }
    pub fn telephone(&self) -> Option<&str> {
        self.telephone.as_deref()
    }
}

impl std::fmt::Display for Contact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\t{} {}", self.email, self.forename, self.surname)
    }
}
