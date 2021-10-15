use clap::{App, Arg};
use contact::{contact::Contact, error::Error, operation::Operation, run};

fn main() -> Result<(), Error> {
    let matches = App::new("Contact")
        .version("0.1")
        .author("Richard Bradshaw")
        .about("A simple contact manager for mutt")
        .subcommand(
            App::new("add")
                .arg(
                    Arg::new("forename")
                        .long("forename")
                        .required(true)
                        .takes_value(true)
                        .short('f'),
                )
                .arg(
                    Arg::new("surname")
                        .long("surname")
                        .required(true)
                        .takes_value(true)
                        .short('s'),
                )
                .arg(
                    Arg::new("email")
                        .long("email")
                        .required(true)
                        .takes_value(true)
                        .short('e'),
                )
                .arg(
                    Arg::new("organisation")
                        .long("organisation")
                        .required(false)
                        .takes_value(true)
                        .short('o'),
                )
                .arg(
                    Arg::new("telephone")
                        .long("telephone")
                        .required(false)
                        .takes_value(true)
                        .short('t'),
                ),
        )
        .subcommand(
            App::new("search").arg(
                Arg::new("needle")
                    .long("needle")
                    .required(true)
                    .takes_value(true)
                    .short('n'),
            ),
        )
        .get_matches();
    let operation = match matches.subcommand() {
        Some(("add", add_matches)) => {
            let mut contact = Contact::new(
                add_matches.value_of("forename").unwrap(),
                add_matches.value_of("surname").unwrap(),
                add_matches.value_of("email").unwrap(),
            )?;
            if let Some(organisation) = matches.value_of("organisation") {
                contact.set_organisation(organisation)?;
            }
            if let Some(telephone) = matches.value_of("telephone") {
                contact.set_telephone(telephone)?;
            }
            Operation::Add(contact)
        }
        Some(("search", search_matches)) => {
            Operation::List(search_matches.value_of("needle").unwrap())
        }
        _ => return Err(Error::NoArg),
    };
    run(operation)
}
