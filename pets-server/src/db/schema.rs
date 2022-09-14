use async_graphql::Context;
use mongodb::bson::{oid::ObjectId, Bson, DateTime, Document};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Pet {
    pub(crate) id: ObjectId,
    pub(crate) name: String,
    pub(crate) birthday: DateTime,
    pub(crate) level: usize,
    pub(crate) experiences: usize,
    pub(crate) kind: Kind,
}

impl Pet {
    pub(crate) fn new(name: String, kind: Kind) -> Self {
        Pet {
            id: ObjectId::new(),
            name,
            birthday: chrono::Utc::now().into(),
            level: 1,
            experiences: 0,
            kind,
        }
    }
}

#[async_graphql::Object]
impl Pet {
    async fn id(&self) -> String {
        self.id.to_string()
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn kind(&self) -> &'static str {
        self.kind.as_str()
    }

    async fn level(&self) -> usize {
        self.level
    }

    async fn experiences(&self) -> usize {
        self.experiences
    }

    async fn birthday(&self) -> chrono::DateTime<chrono::Utc> {
        self.birthday.into()
    }
}

impl From<Pet> for Bson {
    fn from(pet: Pet) -> Self {
        let mut doc = Document::new();
        doc.insert("id", pet.id);
        doc.insert("name", pet.name);
        doc.insert("birthday", pet.birthday);
        doc.insert("level", pet.level as i64);
        doc.insert("experiences", pet.experiences as i64);
        doc.insert("kind", pet.kind.as_str());

        Bson::Document(doc)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum Kind {
    Dog,
    Cat,
    Panda,
    Fox,
}

impl Kind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Kind::Dog => "dog",
            Kind::Cat => "cat",
            Kind::Panda => "panda",
            Kind::Fox => "fox",
        }
    }
}

impl From<String> for Kind {
    fn from(kind: String) -> Self {
        match kind.as_str().trim() {
            "dog" => Kind::Dog,
            "cat" => Kind::Cat,
            "panda" => Kind::Panda,
            "fox" => Kind::Fox,
            _ => panic!("Invalid pet kind"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Ticket {
    pub(crate) description: String,
    pub(crate) expires_at: u64,
}

impl Ticket {
    pub fn new(description: String, expires_at: u64) -> Ticket {
        Ticket {
            description,
            expires_at,
        }
    }
}

impl From<Ticket> for Bson {
    fn from(ticket: Ticket) -> Self {
        let mut doc = Document::new();
        doc.insert("description", ticket.description);
        doc.insert("expires_at", ticket.expires_at as i64);
        Bson::Document(doc)
    }
}

#[async_graphql::Object]
impl Ticket {
    async fn description(&self) -> &str {
        &self.description
    }

    async fn expires_at(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_utc(
            chrono::NaiveDateTime::from_timestamp(self.expires_at as i64, 0),
            chrono::Utc,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct User {
    pub(crate) id: ObjectId,
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) pet: Option<Pet>,
    pub(crate) currency: usize,
    pub(crate) tickets: Vec<Ticket>,
    pub(crate) followers: Vec<ObjectId>,
    pub(crate) followings: Vec<ObjectId>,
}

impl User {
    pub fn new(username: String, email: String, password: String) -> User {
        Self {
            id: ObjectId::new(),
            username,
            email,
            password,
            pet: None,
            currency: 0,
            tickets: Vec::new(),
            followers: Vec::new(),
            followings: Vec::new(),
        }
    }
}

#[async_graphql::Object]
impl User {
    async fn id(&self) -> String {
        self.id.to_string()
    }

    async fn username(&self) -> &str {
        &self.username
    }

    async fn email(&self) -> &str {
        &self.email
    }

    async fn pet(&self) -> Option<Pet> {
        self.pet.clone()
    }

    async fn currency(&self) -> usize {
        self.currency
    }

    async fn tickets(&self) -> Vec<Ticket> {
        self.tickets.clone()
    }

    async fn followers<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<User>, async_graphql::Error> {
        // self.followers.iter().map(|id| id.to_string()).collect();
        todo!()
    }

    async fn followings<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<User>, async_graphql::Error> {
        // self.followers.iter().map(|id| id.to_string()).collect()
        todo!()
    }
}
