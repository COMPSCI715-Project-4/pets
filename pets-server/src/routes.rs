use super::schema::{Kind, Pet, User};
use axum::{response::IntoResponse, Json};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

pub(crate) trait Data:
    Clone + core::fmt::Debug + Serialize + serde::de::DeserializeOwned + 'static
{
}

impl<T> Data for T where
    T: Clone + core::fmt::Debug + Serialize + serde::de::DeserializeOwned + 'static
{
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Response<D: Data> {
    #[serde(skip_serializing_if = "Option::is_none")]
    err: Option<String>,
    #[serde(bound = "D: Data", skip_serializing_if = "Option::is_none")]
    data: Option<D>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RankRecord {
    username: String,
    pet: Pet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CreatePetRequest {
    user_id: ObjectId,
    name: String,
    kind: Kind,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub(crate) struct UpdatePetRequest {
    user_id: ObjectId,
    level: usize,
    experiences: usize,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub(crate) struct DeletePetRequest {
    user_id: ObjectId,
}

pub(crate) async fn register(Json(payload): Json<RegisterRequest>) -> impl IntoResponse {
    todo!()
}

pub(crate) async fn login(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    todo!()
}

pub(crate) async fn rank() -> impl IntoResponse {
    todo!()
}

pub(crate) async fn create_pet(Json(payload): Json<CreatePetRequest>) -> impl IntoResponse {
    todo!()
}

pub(crate) async fn update_pet(Json(payload): Json<UpdatePetRequest>) -> impl IntoResponse {
    todo!()
}

pub(crate) async fn delete_pet(Json(payload): Json<DeletePetRequest>) -> impl IntoResponse {
    todo!()
}
