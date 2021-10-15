use crate::contact::Contact;

pub enum Operation {
    Add(Contact),
    Delete(i64),
    List,
    Search(String),
}
