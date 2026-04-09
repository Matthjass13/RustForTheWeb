use crate::auth::{Claims, Role};
use jsonwebtoken::{DecodingKey, Validation, decode};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};

pub struct AuthenticatedUser {
    pub user_id: i32,
    pub role: Role,
}

pub struct AdminUser;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = req
            .headers()
            .get_one("Authorization")
            .and_then(|h| h.strip_prefix("Bearer "));

        let token = match token {
            Some(t) => t,
            None => return Outcome::Error((Status::Unauthorized, ())),
        };

        let secret = std::env::var("JWT_SECRET").unwrap_or_default();

        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        ) {
            Ok(data) => Outcome::Success(AuthenticatedUser {
                user_id: data.claims.user_id,
                role: data.claims.role,
            }),
            Err(_) => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth = AuthenticatedUser::from_request(req).await;

        match auth {
            Outcome::Success(user) if user.role == Role::Admin => Outcome::Success(AdminUser),
            _ => Outcome::Error((Status::Forbidden, ())),
        }
    }
}
