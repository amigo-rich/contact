use crate::contact::Contact;
use crate::error::{Error, Field};
use rusqlite::{params, Connection};
use std::path::Path;

pub struct Record<T> {
    id: i64,
    record: T,
}

impl<T> Record<T> {
    pub fn new(id: i64, record: T) -> Self {
        Record { id, record }
    }
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn record(&self) -> &T {
        &self.record
    }
}

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn create(p: &Path) -> Result<Self, Error> {
        let sql = r#"
            CREATE TABLE contact (
                id INTEGER PRIMARY KEY,
                forename TEXT NOT NULL,
                surname TEXT NOT NULL,
                email TEXT NOT NULL,
                organisation TEXT,
                telephone TEXT
            )
        "#;
        let mut connection = Connection::open(p)?;
        let transaction = connection.transaction()?;
        transaction.execute(sql, params![])?;
        transaction.commit()?;

        Ok(Database { connection })
    }
    pub fn open(p: &Path) -> Result<Self, Error> {
        if !p.is_file() {
            return Err(Error::NoFile(p.to_path_buf()));
        }
        let connection = Connection::open(p)?;
        Ok(Database { connection })
    }
    pub fn delete_contact(&self, contact_id: i64) -> Result<(), Error> {
        let sql = r#"
            DELETE FROM contact
            WHERE id = ?1
        "#;
        self.connection.execute(sql, params![contact_id])?;
        Ok(())
    }
    pub fn insert_contact(&self, contact: &Contact) -> Result<(), Error> {
        let sql = r#"
            INSERT INTO contact (forename, surname, email, organisation, telephone)
            VALUES (?1, ?2, ?3, ?4, ?5)
        "#;
        self.connection.execute(
            sql,
            params![
                contact.forename(),
                contact.surname(),
                contact.email().to_string(),
                contact.organisation(),
                contact.telephone(),
            ],
        )?;
        Ok(())
    }
    pub fn select_contacts(&self) -> Result<Option<Vec<Record<Contact>>>, Error> {
        let sql = r#"
            SELECT id, forename, surname, email, organisation, telephone
            FROM contact
            ORDER BY surname, forename
        "#;
        let mut statement = self.connection.prepare(sql)?;
        let iterator =
            statement.query_and_then(params![], |row| -> Result<Record<Contact>, Error> {
                let mut contact = Contact::new(row.get(1)?, row.get(2)?, row.get(3)?)?;
                if let Some(organisation) = row.get(4)? {
                    contact.set_organisation(organisation)?;
                }
                if let Some(telephone) = row.get(5)? {
                    contact.set_telephone(telephone)?;
                }
                Ok(Record::<Contact>::new(row.get(0)?, contact))
            })?;
        let mut results: Vec<Record<Contact>> = Vec::new();
        for result in iterator {
            results.push(result?);
        }
        if results.is_empty() {
            return Ok(None);
        }
        Ok(Some(results))
    }
    pub fn select_contact(&self, needle: &str) -> Result<Option<Vec<Contact>>, Error> {
        if needle.is_empty() {
            return Err(Error::Empty(Field::Needle));
        }
        let needle = format!("%{}%", needle);
        let sql = r#"
            SELECT forename, surname, email
            FROM contact
            WHERE LIKE(?1, forename)
            OR LIKE(?2, surname)
            OR LIKE(?3, email)
            OR LIKE(?4, organisation)
            OR LIKE(?5, telephone)
            ORDER BY surname, forename
        "#;
        let mut statement = self.connection.prepare(sql)?;
        let iterator = statement.query_and_then(
            params![&needle, &needle, &needle, &needle, &needle],
            |row| -> Result<Contact, Error> { Contact::new(row.get(0)?, row.get(1)?, row.get(2)?) },
        )?;
        let mut results: Vec<Contact> = Vec::new();
        for result in iterator {
            results.push(result?);
        }
        if results.is_empty() {
            return Ok(None);
        }
        Ok(Some(results))
    }
}
