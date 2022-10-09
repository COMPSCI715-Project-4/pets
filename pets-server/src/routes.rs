use crate::db::schema::Record;

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
pub(crate) struct UpdateAverageStepsRequest {
    pub(crate) token: String,
    pub(crate) average_steps: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UpdateRecordRequest {
    pub(crate) token: String,
    pub(crate) kind: String,
    pub(crate) steps: usize,
    pub(crate) level: usize,
    pub(crate) distance: f64,
    pub(crate) duration: usize,
}

impl<'a> From<&'a UpdateRecordRequest> for Record {
    fn from(req: &'a UpdateRecordRequest) -> Self {
        Self {
            steps: req.steps as i64,
            level: req.level as i64,
            distance: req.distance,
            duration: req.duration as i64,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ResetAverageStepsRequest {
    pub(crate) token: String,
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
    pub(crate) highest_steps: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RankResponse {
    pub(crate) users: Vec<Rank>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Response<D> {
    pub(crate) err: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) data: Option<D>,
}

impl<D> Response<D> {
    pub(crate) fn new() -> Self {
        Response {
            err: None,
            data: None,
        }
    }

    pub(crate) fn with_err(err: String) -> Self {
        Response {
            err: Some(err),
            data: None,
        }
    }

    pub(crate) fn with_data(data: D) -> Self {
        Response {
            err: None,
            data: Some(data),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RankRecord {
    username: String,
    pet: Pet,
}
