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

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub(crate) struct Record {
    pub(crate) steps: i64,
    pub(crate) level: i64,
    pub(crate) distance: f64,
    pub(crate) duration: i64,
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.steps == other.steps && self.level == other.level
    }
}

impl Eq for Record {}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Record {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.steps
            .cmp(&other.steps)
            .then(self.level.cmp(&other.level))
    }
}

impl From<Record> for Bson {
    fn from(record: Record) -> Self {
        let mut doc = Document::new();
        doc.insert("steps", record.steps);
        doc.insert("level", record.level);
        doc.insert("distance", record.distance);
        doc.insert("duration", record.duration);
        Bson::Document(doc)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct User {
    pub(crate) id: ObjectId,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) pet: Pet,
    pub(crate) tickets: Vec<Ticket>,
    pub(crate) average_steps: Option<i64>,
    pub(crate) evolution: Record,
    pub(crate) rank: Record,
    pub(crate) ticket: Record,
}

impl User {
    pub fn new(username: String, password: String) -> User {
        Self {
            id: ObjectId::new(),
            username,
            password,
            pet: Pet::new(),
            tickets: Vec::new(),
            average_steps: None,
            evolution: Record::default(),
            rank: Record::default(),
            ticket: Record::default(),
        }
    }
}

impl From<User> for Bson {
    fn from(user: User) -> Self {
        let mut doc = Document::new();
        doc.insert("id", user.id);
        doc.insert("username", user.username);
        doc.insert("password", user.password);
        doc.insert("pet", user.pet);
        doc.insert("tickets", user.tickets);
        doc.insert("average_steps", user.average_steps);
        doc.insert("evolution", user.evolution);
        doc.insert("rank", user.rank);
        doc.insert("ticket", user.ticket);
        Bson::Document(doc)
    }
}
