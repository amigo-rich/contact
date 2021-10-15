use crate::contact::Contact;

pub enum Operation {
    Add(Contact),
    List(String),
}
