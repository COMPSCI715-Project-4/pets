use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub(crate) const PET_EXPERIENCE_FACTOR: usize = 200;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Pet {
    pub(crate) name: String,
    pub(crate) birthday: i64,
    pub(crate) level: usize,
    pub(crate) experiences: usize,
    pub(crate) kind: Kind,
}

impl Pet {
    pub(crate) fn new(name: String, kind: Kind) -> Self {
        Pet {
            name,
            birthday: Utc::now().timestamp(),
            level: 1,
            experiences: 0,
            kind,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum Kind {
    Dog,
    Cat,
    Panda,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct User {
    pub(crate) id: ObjectId,
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) salt: String,
    pub(crate) pet: Option<Pet>,
}
