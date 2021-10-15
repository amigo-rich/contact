use clap::Clap;
use contact::{contact::Contact, error::Error, operation::Operation, run};

#[derive(Clap)]
#[clap(version = "0.1", author = "Richard Bradshaw <merryidleness@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Add(Add),
    Search(Search),
}

/// Add a new contact
#[derive(Clap)]
struct Add {
    /// The contact's forename
    #[clap(short, long)]
    forename: String,
    /// The contact's surname
    #[clap(short, long)]
    surname: String,
    /// The contact's email-address
    #[clap(short, long)]
    email: String,
    /// The contact's organisation
    #[clap(short, long)]
    organisation: Option<String>,
    /// The contact's telephone-number
    #[clap(short, long)]
    telephone: Option<String>,
}

/// Search for a contact
#[derive(Clap)]
struct Search {
    /// The needle to search for. Search includes forename, surname, email, organisation
    /// and telephone.
    #[clap(short, long)]
    needle: String,
}

fn main() -> Result<(), Error> {
    let opts = Opts::parse();
    let operation = match opts.subcmd {
        SubCommand::Add(c) => {
            let mut contact = Contact::new(c.forename, c.surname, c.email)?;
            if let Some(organisation) = c.organisation {
                contact.set_organisation(organisation)?;
            }
            if let Some(telepone) = c.telephone {
                contact.set_telephone(telepone)?;
            }
            Operation::Add(contact)
        }
        SubCommand::Search(s) => Operation::List(s.needle),
    };
    run(operation)
}
