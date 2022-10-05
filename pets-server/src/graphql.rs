use crate::{
    db::schema::{Pet, Ticket, User},
    routes::{UserResponse, LoginRequest, Response, SignupRequest, UpdatePetRequest, CreateTicketRequest, FetchTicketsRequest, Rank},
    DB_CLIENT, CONFIG,
};

use async_graphql::futures_util::StreamExt;
use axum::{
    response::IntoResponse, http::StatusCode, Json, Form,
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use mongodb::bson::doc;
use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String, // Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Issued at (as UTC timestamp)
    iss: String, // Issuer
    nbf: usize, // Not Before (as UTC timestamp)
    sub: String, // Subject (whom token refers to)
}

#[inline]
fn gen_jwt(username: String, secret: &str) -> Result<String, async_graphql::Error> {
    let now = Utc::now();
    let claims = Claims {
        aud: "COMPSCI 715 classmates".to_owned(),
        nbf: now.timestamp() as usize,
        iat: now.timestamp() as usize,
        iss: "Go Walkies".to_owned(),
        exp: now.timestamp() as usize + (3600 * 24 * 30),
        sub: username,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| {
        tracing::error!(err=?e);
        async_graphql::Error::new(format!("fail to authenticate: {}", e))
    })
}

#[inline]
fn parse_jwt(token: String, secret: &str) -> Result<Claims, async_graphql::Error> {
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::default()),
    )
    .map(|d| d.claims)
    .map_err(|e| {
        tracing::error!(err=?e);
        async_graphql::Error::new(format!("invalid token: {}", e))
    })
}

#[inline]
fn hash_password(password: &[u8], salt: impl AsRef<str>) -> Result<String, async_graphql::Error> {
    Pbkdf2
        .hash_password(password, salt.as_ref())
        .map(|x| x.to_string())
        .map_err(|e| {
            tracing::error!(err=?e);
            async_graphql::Error::new(format!("{}", e))
        })
}

#[inline]
fn verify_password(password: &[u8], password_hash: &str) -> Result<(), async_graphql::Error> {
    // Verify password against PHC string
    PasswordHash::new(password_hash)
        .and_then(|parsed_hash| Pbkdf2.verify_password(password, &parsed_hash))
        .map_err(|e| {
            tracing::error!(err=?e);
            async_graphql::Error::new("wrong email or password")
        })
}


pub(crate) async fn login(Form(req): Form<LoginRequest>) -> impl IntoResponse {
    let client = DB_CLIENT.get().unwrap();
    let cfg = CONFIG.get().unwrap();
    let users = client
        .database(&cfg.db_name)
        .collection::<User>("users");
    
    let filter = doc! {
        "username": req.username.clone(),
    };

    match users.find_one(filter, None).await {
        Ok(Some(user)) => {
            match verify_password(req.password.as_bytes(), &user.password).and_then(|_| {
                gen_jwt(user.username.clone(), &cfg.secret)
                    .map(|token| (StatusCode::OK, Json(Response {
                        err: None,
                        data: Some(UserResponse { token, user }),
                    })))
            }) {
                Ok(res) => res,
                Err(e) => {
                    tracing::error!(err=?e);
                    (StatusCode::BAD_REQUEST, Json(Response {
                        err: Some(format!("{:?}", e)),
                        data: None,
                    }))
                }
            }
        },
        Ok(None) => (StatusCode::BAD_REQUEST, Json(Response { err: Some("user not found".to_string()), data: None })), 
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Response { err: Some(e.to_string()), data: None })),
    }
}

pub(crate) async fn signup(Form(req): Form<SignupRequest>) -> impl IntoResponse {
    let client = DB_CLIENT.get().unwrap();
    let cfg = CONFIG.get().unwrap();
    let users = client.database(&cfg.db_name).collection::<User>("users");
    let filter = doc! {
        "username": req.username.clone(),
    };

    if let Ok(Some(_)) = users
    .find_one(filter, None)
    .await {
        return (StatusCode::BAD_REQUEST, Json(Response { err: Some("user already exists".to_string()), data: None }));
    }

    if let Ok(user) = hash_password(req.password.as_bytes(), SaltString::generate(&mut OsRng))
        .map(|password| User::new(req.username, password)) {
        match users
        .insert_one(&user, None)
        .await {
            Ok(_) => {
                match gen_jwt(user.username.clone(), &cfg.secret)
                .map(|token| UserResponse { user, token }) {
                    Ok(user) => (StatusCode::OK, Json(Response { err: None, data: Some(user) })),
                    Err(e) => {
                        tracing::error!(err=?e);
                        (StatusCode::INTERNAL_SERVER_ERROR, Json(Response { err: Some(format!("{:?}", e)), data: None }))
                    }
                }
            },
            Err(e) => {
                tracing::error!(err=?e);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(Response {
                    err: Some(format!("{:?}", e)),
                    data: None,
                }))
            },
        }
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(Response { err: Some("internal server error".to_string()), data: None }))
    }
}


pub(crate) async fn update_pet(Form(req): Form<UpdatePetRequest>) -> impl IntoResponse { 
    let client = DB_CLIENT.get().unwrap();
    let cfg = CONFIG.get().unwrap();
    let jwt = match parse_jwt(req.token, &cfg.secret) {
        Ok(jwt) => jwt,
        Err(e) => {
            tracing::error!(err=?e);
            return (StatusCode::BAD_REQUEST, Json(Response { err: Some(format!("{:?}", e)), data: None }));
        }
    };

    let users = client
        .database(&cfg.db_name)
        .collection::<User>("users");
    
    let filter = doc! {
        "username": jwt.sub,
    };

    match users.find_one(filter.clone(), None).await {
        Ok(Some(user)) => {
            let mut pet: Pet = user.pet;
            pet.level = req.level;
            let update = doc! {
                "$set": {
                    "pet": Some(pet.clone()),
                }
            };

            match users.update_one(filter, update, None).await {
                Ok(_) => (StatusCode::OK, Json(Response { err: None, data: Some(pet) })),
                Err(e) => {
                    tracing::error!(err=?e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(Response { err: Some(format!("{:?}", e)), data: None }))
                }
            }
        },
        Ok(None) => {
            (StatusCode::BAD_REQUEST, Json(Response { err: Some("user not found".to_string()), data: None }))
        },
        Err(e) => {
            tracing::error!(err=?e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Response { err: Some(format!("{:?}", e)), data: None }))
        },
    }
}

pub(crate) async fn create_ticket(Form(req): Form<CreateTicketRequest>) -> impl IntoResponse {

    let client = DB_CLIENT.get().unwrap();
    let cfg = CONFIG.get().unwrap();
    let jwt = match parse_jwt(req.token, &cfg.secret) {
        Ok(jwt) => jwt,
        Err(e) => {
            tracing::error!(err=?e);
            return (StatusCode::BAD_REQUEST, Json(Response { err: Some(format!("{:?}", e)), data: None }));
        }
    };

    let users = client
        .database(&cfg.db_name)
        .collection::<User>("users");
    
    let filter = doc! {
        "username": jwt.sub,
    };

    match users.find_one(filter.clone(), None).await {
        Ok(Some(mut user)) => {
            user.tickets.push(Ticket::new(req.description, req.expires_at, req.level));
            let update = doc! {
                "$set": {
                    "tickets": user.tickets.clone(),
                }
            };

            match users.update_one(filter, update, None).await {
                Ok(_) => (StatusCode::OK, Json(Response { err: None, data: Some(user.tickets) })),
                Err(e) => {
                    tracing::error!(err=?e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(Response { err: Some(format!("{:?}", e)), data: None }))
                }
            }
        },
        Ok(None) => {
            (StatusCode::BAD_REQUEST, Json(Response { err: Some("user not found".to_string()), data: None }))
        },
        Err(e) => {
            tracing::error!(err=?e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Response { err: Some(format!("{:?}", e)), data: None }))
        },
    } 
}

pub(crate) async fn fetch_tickets(Form(req): Form<FetchTicketsRequest>) -> impl IntoResponse {
    let client = DB_CLIENT.get().unwrap();
    let cfg = CONFIG.get().unwrap();
    let jwt = match parse_jwt(req.token, &cfg.secret) {
        Ok(jwt) => jwt,
        Err(e) => {
            tracing::error!(err=?e);
            return (StatusCode::BAD_REQUEST, Json(Response { err: Some(format!("{:?}", e)), data: None }));
        }
    };

    let users = client
        .database(&cfg.db_name)
        .collection::<User>("users");
    
    let filter = doc! {
        "username": jwt.sub,
    };

    match users.find_one(filter.clone(), None).await {
        Ok(Some(user)) => {
            let tickets = user.tickets.into_iter().filter(|t| {
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() < t.expires_at
            });
            (StatusCode::OK, Json(Response { err: None, data: Some(tickets.collect::<Vec<Ticket>>()) }))
        },
        Ok(None) => {
            (StatusCode::BAD_REQUEST, Json(Response { err: Some("user not found".to_string()), data: None }))
        },
        Err(e) => {
            tracing::error!(err=?e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Response { err: Some(format!("{:?}", e)), data: None }))
        },
    }
}

pub(crate) async fn rank() -> impl IntoResponse {
    let client = DB_CLIENT.get().unwrap();
    let cfg = CONFIG.get().unwrap();
    let users = client
        .database(&cfg.db_name)
        .collection::<User>("users");
    
    match users.find(doc!{}, None).await {
        Ok(mut users) => {
            let mut vec = Vec::new();
            while let Some(user) = users.next().await {
                match user {
                    Ok(user) => {
                        vec.push(Rank {
                            username: user.username,
                            level: user.pet.level,
                        });
                        if vec.len() > 10 {
                            break;
                        }
                    },
                    Err(e) => {
                        tracing::error!(err=?e);
                        return (StatusCode::INTERNAL_SERVER_ERROR, Json(Response { err: Some(format!("{:?}", e)), data: None }));
                    }
                }
            }
            (StatusCode::OK, Json(Response { err: None, data: Some(vec) })) 
        },
        Err(e) => {
            tracing::error!(err=?e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Response { err: Some(format!("{:?}", e)), data: None })) 
        },
    }
}