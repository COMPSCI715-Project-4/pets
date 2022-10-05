use super::db::schema::{Pet, User};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LoginRequest {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SignupRequest {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CreateTicketRequest {
    pub(crate) description: String,
    pub(crate) level: usize,
    pub(crate) expires_at: u64,
    pub(crate) token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct FetchTicketsRequest {
    pub(crate) token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UpdatePetRequest {
    pub(crate) token: String,
    pub(crate) level: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UserResponse {
    pub(crate) user: User,
    pub(crate) token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Rank {
    pub(crate) username: String,
    pub(crate) level: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RankResponse {
    pub(crate) users: Vec<Rank>,
}

pub(crate) trait Data:
    Clone + core::fmt::Debug + Serialize + serde::de::DeserializeOwned + Send + Sync + 'static
{
}

impl<T> Data for T where
    T: Send + Sync + Clone + core::fmt::Debug + Serialize + serde::de::DeserializeOwned + 'static
{
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Response<D: Data> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) err: Option<String>,
    #[serde(bound = "D: Data", skip_serializing_if = "Option::is_none")]
    pub(crate) data: Option<D>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RankRecord {
    username: String,
    pet: Pet,
}
