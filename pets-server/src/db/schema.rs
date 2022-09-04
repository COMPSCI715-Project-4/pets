use async_graphql::Context;
use mongodb::bson::{oid::ObjectId, DateTime};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Food {
    pub(crate) name: String,
    pub(crate) r#type: FoodType,
    pub(crate) expires_at: u64,
    pub(crate) price: u64,
}

#[async_graphql::Object]
impl Food {
    async fn name(&self) -> &str {
        &self.name
    }
    async fn r#type(&self) -> &'static str {
        self.r#type.as_str()
    }
    async fn expires_at(&self) -> u64 {
        self.expires_at
    }
    async fn price(&self) -> u64 {
        self.price
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub(crate) enum FoodType {
    Apple,
    Banana,
    Orange,
}

impl FoodType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            FoodType::Apple => "apple",
            FoodType::Banana => "banana",
            FoodType::Orange => "orange",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct User {
    pub(crate) id: ObjectId,
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) pets: Vec<Pet>,
    pub(crate) currency: usize,
    pub(crate) foods: Vec<Food>,
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
            pets: Vec::new(),
            currency: 0,
            foods: Vec::new(),
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

    async fn pets(&self) -> Vec<Pet> {
        self.pets.clone()
    }

    async fn currency(&self) -> usize {
        self.currency
    }

    async fn foods(&self) -> Vec<Food> {
        self.foods.clone()
    }

    async fn followers<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<User>, async_graphql::Error> {
        // self.followers.iter().map(|id| id.to_string()).collect();
        todo!()
    }

    async fn followings<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<User>, async_graphql::Error> {
        // self.followers.iter().map(|id| id.to_string()).collect()
        todo!()
    }
}
