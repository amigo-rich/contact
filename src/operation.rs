use crate::contact::Contact;

pub enum Operation {
    Add(Contact),
    List,
    Search(String),
}
