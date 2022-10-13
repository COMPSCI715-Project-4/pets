use mongodb::bson::{Bson, Document};
use serde::{Deserialize, Serialize};

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
        match self.steps.cmp(&other.steps) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => self.level.cmp(&other.level),
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater, 
        }
            
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
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) average_steps: Option<i64>,
    pub(crate) evolution: Record,
    pub(crate) rank: Record,
    pub(crate) ticket: Record,
}

impl User {
    pub fn new(username: String, password: String) -> User {
        Self {
            username,
            password,
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
        doc.insert("username", user.username);
        doc.insert("password", user.password);
        doc.insert("average_steps", user.average_steps);
        doc.insert("evolution", user.evolution);
        doc.insert("rank", user.rank);
        doc.insert("ticket", user.ticket);
        Bson::Document(doc)
    }
}
