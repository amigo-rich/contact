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
    pub fn insert_contact(&self, contact: &Contact) -> Result<i64, Error> {
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
        Ok(self.connection.last_insert_rowid())
    }
    pub fn select_contacts(&self) -> Result<Option<Vec<Record<Contact>>>, Error> {
        let sql = r#"
            SELECT id, forename, surname, email, organisation, telephone
            FROM contact
            ORDER BY surname, forename
        "#;
        let mut statement = self.connection.prepare(sql)?;
        let iterator = statement.query_map(params![], |row| {
            let mut contact = Contact::new(row.get(1)?, row.get(2)?, row.get(3)?).unwrap();
            if let Some(organisation) = row.get(4)? {
                contact.set_organisation(organisation).unwrap();
            }
            if let Some(telephone) = row.get(5)? {
                contact.set_telephone(telephone).unwrap();
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
        // yuck and unsafe! But have to do it this way to work around rusqlite and 'like'
        // binding
        let sql = format!(
            r#"
                SELECT forename, surname, email
                FROM contact
                WHERE LIKE('%{}%', forename)
                OR LIKE('%{}%', surname)
                OR LIKE('%{}%', email)
                OR LIKE('%{}%', organisation)
                OR LIKE('%{}%', telephone)
                ORDER BY surname, forename
            "#,
            needle, needle, needle, needle, needle
        );
        let mut statement = self.connection.prepare(&sql)?;
        let iterator = statement.query_map(params![], |row| {
            Ok(Contact::new(row.get(0)?, row.get(1)?, row.get(2)?).unwrap())
        })?;
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
