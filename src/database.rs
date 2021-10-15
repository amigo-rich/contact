use crate::contact::Contact;
use crate::error::{Error, Field};
use rusqlite::{params, Connection};
use std::path::Path;

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
            let forename: String = row.get(0)?;
            let surname: String = row.get(1)?;
            let email: String = row.get(2)?;
            Ok(Contact::new(&forename, &surname, &email).unwrap())
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
