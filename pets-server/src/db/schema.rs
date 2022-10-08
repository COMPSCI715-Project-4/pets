use mongodb::bson::{oid::ObjectId, Bson, DateTime, Document};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Pet {
    pub(crate) id: ObjectId,
    pub(crate) birthday: DateTime,
    pub(crate) level: usize,
}

impl Pet {
    pub(crate) fn new() -> Self {
        Pet {
            id: ObjectId::new(),
            birthday: chrono::Utc::now().into(),
            level: 1,
        }
    }
}

impl From<Pet> for Bson {
    fn from(pet: Pet) -> Self {
        let mut doc = Document::new();
        doc.insert("id", pet.id);
        doc.insert("birthday", pet.birthday);
        doc.insert("level", pet.level as i64);
        Bson::Document(doc)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Ticket {
    pub(crate) description: String,
    pub(crate) expires_at: u64,
    pub(crate) level: usize,
}

impl Ticket {
    pub fn new(description: String, expires_at: u64, level: usize) -> Ticket {
        Ticket {
            description,
            expires_at,
            level,
        }
    }
}

impl From<Ticket> for Bson {
    fn from(ticket: Ticket) -> Self {
        let mut doc = Document::new();
        doc.insert("description", ticket.description);
        doc.insert("expires_at", ticket.expires_at as i64);
        doc.insert("level", ticket.level as i64);
        Bson::Document(doc)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct User {
    pub(crate) id: ObjectId,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) level: usize,
    pub(crate) pet: Pet,
    pub(crate) tickets: Vec<Ticket>,
    pub(crate) average_steps: Option<i64>,
    pub(crate) highest_steps: i64,
}

impl User {
    pub fn new(username: String, password: String) -> User {
        Self {
            id: ObjectId::new(),
            username,
            password,
            pet: Pet::new(),
            tickets: Vec::new(),
            level: 1,
            average_steps: None,
            highest_steps: 0,
        }
    }
}

impl From<User> for Bson {
    fn from(user: User) -> Self {
        let mut doc = Document::new();
        doc.insert("id", user.id);
        doc.insert("username", user.username);
        doc.insert("password", user.password);
        doc.insert("level", user.level as i64);
        doc.insert("pet", user.pet);
        doc.insert("tickets", user.tickets);
        doc.insert("average_steps", user.average_steps);
        doc.insert("highest_steps", user.highest_steps);
        Bson::Document(doc)
    }
}
