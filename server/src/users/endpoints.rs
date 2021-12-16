use std::fmt::{Debug, Display};

use crate::users::user_db::UserDb;
use crate::users::{hash_password, verify_password};
use actix_identity::Identity;
use actix_web::HttpResponse;
use actix_web::{web, Responder};
use anyhow::anyhow;
use anyhow::Result as AnyResult;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct LoginArgs {
    login: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    success: bool,
    error: Option<String>,
}

impl LoginResponse {
    pub fn ok() -> LoginResponse {
        LoginResponse {
            success: true,
            error: None,
        }
    }

    pub fn error(error: impl Debug) -> LoginResponse {
        LoginResponse {
            success: false,
            error: Some(format!("{:?}", error)),
        }
    }

    pub fn error_display(error: impl Display) -> LoginResponse {
        LoginResponse {
            success: false,
            error: Some(format!("{}", error)),
        }
    }
}

async fn register_account_core(user_db: &UserDb, login: &str, password: &str) -> AnyResult<bool> {
    let password: String = password.into();
    let password_hash =
        web::block(move || hash_password(&password).map_err(|e| anyhow!(e))).await??;

    debug!(%password_hash, "Generated password hash");

    Ok(user_db.register(login, &password_hash).await?)
}

#[instrument(
    name = "User Register",
    skip(args, user_db),
    fields(
        login = %args.login,
    )
    )]
pub async fn register(args: web::Json<LoginArgs>, user_db: web::Data<UserDb>) -> HttpResponse {
    match register_account_core(&user_db, &args.login, &args.password).await {
        Ok(success) => {
            if success {
                HttpResponse::Ok().json(LoginResponse::ok())
            } else {
                HttpResponse::BadRequest().json(LoginResponse::error_display("Account already exists"))
            }
        }
        Err(err) => HttpResponse::InternalServerError().json(LoginResponse::error(err)),
    }
}

async fn login_core(
    id: Identity,
    user_db: &UserDb,
    login: &str,
    password: &str,
) -> AnyResult<LoginResponse> {
    let user_info = user_db.get_user_by_login(login).await?;
    let valid_password =
        verify_password(password, &user_info.password_hash).map_err(|e| anyhow!(e))?;
    if !valid_password {
        return Ok(LoginResponse::error_display("Invalid password"));
    }

    let session_id = Uuid::new_v4().to_string();
    let valid_until = Utc::now() + Duration::days(2);
    user_db
        .create_session(&session_id, user_info.id, valid_until)
        .await?;

    id.remember(session_id);

    Ok(LoginResponse::ok())
}

#[instrument(
    name = "User Login",
    skip(id, args, user_db),
    fields(
        login = %args.login,
    )
    )]
pub async fn login(
    id: Identity,
    args: web::Json<LoginArgs>,
    user_db: web::Data<UserDb>,
) -> HttpResponse {
    match login_core(id, &user_db, &args.login, &args.password).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::InternalServerError().json(LoginResponse::error(err)),
    }
}

#[instrument(name = "User Logout", skip(id, user_db))]
pub async fn logout(id: Identity, user_db: web::Data<UserDb>) -> HttpResponse {
    if let Some(id) = id.identity() {
        info!(%id, "Removing session");
        if let Err(error) = user_db.remove_session(&id).await {
            error!(?error, "Failed to remove session");
        }
    }
    id.forget();

    HttpResponse::Ok().finish()
}
