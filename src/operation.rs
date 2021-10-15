use crate::contact::Contact;

pub enum Operation<'a> {
    Add(Contact),
    List(&'a str),
}
