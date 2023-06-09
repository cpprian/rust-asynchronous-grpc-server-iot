use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, RwLock};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tonic::{Status, Request, Response};

use crate::iot_manifest::{AuthRequest, AuthResponse, VerifyTokenRequest, VerifyTokenResponse};
use crate::iot_manifest::auth_service_server::AuthService;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    username: String,
    password: String,
    role: Role,
}

pub struct AuthServer {
    pub users: Arc::<RwLock<HashMap<String, (String, Role)>>>,
    pub secret_key: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Copy, Clone)]
pub enum Role {
    Admin, 
    User,
}

impl From<i32> for Role {
    fn from(role: i32) -> Self {
        println!("Role: {}", role);
        match role {
            0 => Role::Admin,
            1 => Role::User,
            _ => panic!("Invalid role"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub(crate) username: String,
    pub(crate) exp: usize,
    pub(crate) role: Role,
}

impl AuthServer {
    pub fn new() -> Self {
        let users = serde_json::from_str::<Vec<User>>(
            &fs::read_to_string("data/user_data.json").unwrap(),
        )
        .unwrap()
        .into_iter()
        .map(|userdata| (userdata.username.clone(), (userdata.password, userdata.role)))
        .collect::<HashMap<String, (String, Role)>>();

        let secret_key = b"secret_key".to_vec();

        println!("Users: {:?}", users);

        Self {
            users: Arc::new(RwLock::new(users)),
            secret_key,
        }
    }

    fn authenticate(&self, username: &str, password: &str) -> Result<Role, Status> {
        let users = self.users.read().unwrap();

        match users.get(username) {
            Some((expected_password, role)) if expected_password == password => {
                println!("Role: {:?}", role);
                Ok(*role)
            },
            _ => Err(Status::unauthenticated("Invalid username/password")),
        }
    }

    fn generate_token(&self, username: &str, role: Role) -> String {
        let expiration = 10;
        let claims = Claims {
            username: username.to_owned(),
            exp: (chrono::Utc::now() + chrono::Duration::seconds(expiration as i64)).timestamp() as usize,
            role,
        };

        encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(&self.secret_key),
        )
        .unwrap()
    }

    fn validate_token(&self, token: &str, expected_role: Role) -> Result<(), Status> {
        let validation = Validation::default();

        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(&self.secret_key),
            &validation,
        ) {
            Ok(decoded) if decoded.claims.role == expected_role => Ok(()),
            _ => Err(Status::unauthenticated("Invalid token")),
        }
    }
}

#[tonic::async_trait]
impl AuthService for AuthServer {
    async fn auth(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let auth_request = request.into_inner();

        let role = self.authenticate(&auth_request.username, &auth_request.password)?;
        let token = self.generate_token(&auth_request.username, role);

        let response = AuthResponse {
            token,
            role: role as i32,
        };

        println!("Response: {:?}", response);

        Ok(Response::new(response))
    }

    async fn verify_token(
        &self,
        request: Request<VerifyTokenRequest>,
    ) -> Result<Response<VerifyTokenResponse>, Status> {
        let verify_request = request.into_inner();

        match self.validate_token(&verify_request.token, verify_request.role.into()) {
            Ok(_) => Ok(Response::new(VerifyTokenResponse { valid: true })),
            Err(_) => Ok(Response::new(VerifyTokenResponse { valid: false })),
        }
    }
}