use crate::{
    db::schema::{Pet, User},
    routes::{Response, UserResponse},
    Config,
};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Context, EmptyMutation, EmptySubscription, Result, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use mongodb::{bson::doc, Client};
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
fn gen_jwt(id: String, secret: &str) -> Result<String, async_graphql::Error> {
    let now = Utc::now();
    let claims = Claims {
        aud: "COMPSCI 715 classmates".to_owned(),
        nbf: now.timestamp() as usize,
        iat: now.timestamp() as usize,
        iss: "Go Walkies".to_owned(),
        exp: now.timestamp() as usize + (3600 * 24 * 30),
        sub: id,
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
        .and_then(|parsed_hash| {
            Pbkdf2
            .verify_password(password, &parsed_hash)
        })
        .map_err(|e| {
            tracing::error!(err=?e);
            async_graphql::Error::new("wrong email or password")
        })
}

pub(crate) struct Graphql;

#[async_graphql::Object]
impl Graphql {
    async fn register<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        #[graphql(validator(min_length = 4, max_length = 64))] username: String,
        #[graphql(validator(email))] email: String,
        #[graphql(validator(min_length = 8, max_length = 16))] password: String,
    ) -> Result<Response<UserResponse>> {
        let client = ctx.data::<Client>()?;
        let cfg = ctx.data::<Config>()?;
        let users = client.database(&cfg.db_name).collection::<User>("users");
        let filter = doc! {
            "email": email.clone(),
        };

        if users
            .find_one(filter, None)
            .await
            .map_err(|e| {
                tracing::error!(err=?e);
                async_graphql::Error::new(format!("{}", e))
            })?
            .is_some()
        {
            return Err(async_graphql::Error::new("user already exists"));
        }

        let user = hash_password(password.as_bytes(), SaltString::generate(&mut OsRng))
            .map(|password| User::new(username, email, password))?;
        users
            .insert_one(&user, None)
            .await
            .map_err(|e| {
                tracing::error!(err=?e);
                async_graphql::Error::new(format!("{}", e))
            })
            .and_then(|_| {
                gen_jwt(user.id.to_string(), &cfg.secret)
                    .map(|token| Response::new(UserResponse { user, token }))
            })
    }

    async fn login<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        #[graphql(validator(email))] email: String,
        #[graphql(validator(min_length = 8, max_length = 16))] password: String,
    ) -> Result<Response<UserResponse>> {
        let cfg = ctx.data::<Config>()?;
        let users = ctx
            .data::<Client>()?
            .database(&cfg.db_name)
            .collection::<User>("users");
        let filter = doc! {
            "email": email.clone(),
        };

        if let Some(user) = users.find_one(filter, None).await.map_err(|e| {
            tracing::error!(err=?e);
            async_graphql::Error::new(format!("{}", e))
        })? {
            verify_password(password.as_bytes(), &user.password).and_then(|_| {
                gen_jwt(user.id.to_string(), &cfg.secret)
                    .map(|token| Response::new(UserResponse { token, user }))
            })
        } else {
            Err(async_graphql::Error::new("wrong email or password"))
        }
    }

    async fn follow<'ctx>(&self, ctx: &Context<'ctx>, token: String, user_id: String) -> Result<Response<UserResponse>, async_graphql::Error> {
        todo!()
    }

    async fn unfollow<'ctx>(&self, ctx: &Context<'ctx>, token: String, user_id: String) -> Result<Response<UserResponse>, async_graphql::Error> {
        todo!()
    }

    async fn create_pet<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        token: String,
        #[graphql(validator(min_length = 4, max_length = 64))] name: String,
        kind: String,
    ) -> Result<Response<Pet>> {
        todo!()
    }

    async fn update_pet<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        token: String,
        pet_id: String,
        #[graphql(validator(min_length = 4, max_length = 64))] name: Option<String>,
        level: Option<usize>,
        experiences: Option<usize>,
    ) -> Result<Response<Pet>> {
        todo!()
    }

    async fn delete_pet<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        token: String,
        pet_id: String,
    ) -> Result<Response<Pet>> {
        todo!()
    }
}

pub(crate) async fn graphql_handler(
    schema: Extension<Schema<Graphql, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub(crate) async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}
