pub mod contact;
mod database;
use database::Database;
use std::env;
pub mod error;
use error::Error;
pub mod operation;
use operation::Operation;
use std::path::Path;

const ENV_KEY: &str = "CONTACT_DB";

fn get_database_env() -> Result<String, Error> {
    for (k, v) in env::vars() {
        if k == ENV_KEY {
            return Ok(v);
        }
    }
    Err(Error::Env)
}

fn get_database() -> Result<Database, Error> {
    let maybe_path = get_database_env()?;
    let path = Path::new(&maybe_path);
    let database = match path.is_file() {
        true => Database::open(path)?,
        false => Database::create(path)?,
    };
    Ok(database)
}

pub fn run(operation: Operation) -> Result<(), Error> {
    let database = get_database()?;
    match operation {
        Operation::Add(contact) => database.insert_contact(&contact),
        Operation::Delete(id) => database.delete_contact(id),
        Operation::List => {
            if let Some(results) = database.select_contacts()? {
                println!(
                    "{:<5}{:<10}{:<10}{:<35}{:<15}{:<10}",
                    "Id", "Forename", "Surname", "Email", "Organisation", "Telephone",
                );
                for result in results {
                    println!(
                        "{:<5}{:<10}{:<10}{:<35}{:<15}{:<10}",
                        result.id(),
                        result.record().forename(),
                        result.record().surname(),
                        result.record().email().to_string(),
                        result.record().organisation().unwrap_or("None"),
                        result.record().telephone().unwrap_or("None"),
                    );
                }
            }
            Ok(())
        }
        Operation::Search(needle) => {
            if let Some(results) = database.select_contact(&needle)? {
                println!("{} result(s):", results.len());
                for result in results {
                    println!("{}", result);
                }
            }
            Ok(())
        }
    }
}
