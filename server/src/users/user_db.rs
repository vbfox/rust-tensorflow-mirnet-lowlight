use anyhow::Result as AnyResult;
use chrono::{DateTime, TimeZone, Utc};
use futures::lock::{Mutex, MutexLockFuture};
use rusqlite::{params, Connection};
use std::sync::Arc;
use tracing::{info, instrument};

#[derive(Clone)]
pub struct UserDb {
    connection: Arc<Mutex<Connection>>,
}

pub struct UserInfo {
    pub id: i32,
    pub login: String,
    pub password_hash: String,
}

pub struct SessionInfo {
    pub id: String,
    pub user_id: i32,
    pub valid_until: DateTime<Utc>,
}

impl UserDb {
    pub fn new(connection: Connection) -> Self {
        Self {
            connection: Arc::new(Mutex::new(connection)),
        }
    }

    pub fn connection(&self) -> MutexLockFuture<Connection> {
        self.connection.lock()
    }

    #[instrument(name = "UserDb::initialize", skip(self))]
    pub async fn initialize(&self) -> AnyResult<()> {
        info!("Initializing database");

        let connection = self.connection().await;

        connection.execute(
            "create table if not exists users (
                 id integer primary key,
                 login text not null unique,
                 password_hash text not null
             )",
            [],
        )?;

        connection.execute(
            "create table if not exists sessions (
                 id string primary key,
                 user_id integer not null,
                 valid_until integer not null
             )",
            [],
        )?;

        Ok(())
    }

    #[instrument(name = "UserDb::register", skip(self, password_hash))]
    pub async fn register(
        &self,
        login: &str,
        password_hash: &str,
    ) -> Result<bool, rusqlite::Error> {
        let connection = self.connection().await;

        let result = connection.execute(
            "INSERT INTO users(login, password_hash) VALUES (?1, ?2)",
            params![login, password_hash],
        );

        if let Err(error) = result {
            match error {
                rusqlite::Error::SqliteFailure(sql_error, _) => {
                    if sql_error.code == rusqlite::ErrorCode::ConstraintViolation {
                        Ok(false)
                    } else {
                        Err(error)
                    }
                }
                _ => Err(error),
            }
        } else {
            Ok(true)
        }
    }

    #[instrument(name = "UserDb::get_user_by_login", skip(self))]
    pub async fn get_user_by_login(&self, login: &str) -> Result<UserInfo, rusqlite::Error> {
        let connection = self.connection().await;

        connection.query_row(
            "SELECT id, password_hash FROM users WHERE login=?1",
            [login],
            |r| {
                let id: i32 = r.get(0)?;
                let password_hash: String = r.get(1)?;
                Ok(UserInfo {
                    id,
                    login: login.into(),
                    password_hash,
                })
            },
        )
    }

    #[instrument(name = "UserDb::create_session", skip(self))]
    pub async fn create_session(
        &self,
        id: &str,
        user_id: i32,
        valid_until: DateTime<Utc>,
    ) -> Result<(), rusqlite::Error> {
        let connection = self.connection().await;

        connection.execute(
            "INSERT INTO sessions(id, user_id, valid_until) VALUES (?1, ?2, ?3)",
            params![id, user_id, valid_until.timestamp()],
        )?;

        Ok(())
    }

    #[instrument(name = "UserDb::remove_session", skip(self))]
    pub async fn remove_session(&self, id: &str) -> Result<(), rusqlite::Error> {
        let connection = self.connection().await;

        connection.execute("DELETE FROM sessions WHERE id=?1", params![id])?;

        Ok(())
    }

    #[instrument(name = "UserDb::get_session_by_id", skip(self))]
    pub async fn get_session_by_id(&self, id: &str) -> Result<SessionInfo, rusqlite::Error> {
        let connection = self.connection().await;

        connection.query_row(
            "SELECT user_id, valid_until FROM users WHERE id=?1",
            [id],
            |r| {
                let user_id: i32 = r.get(0)?;
                let valid_until: i32 = r.get(1)?;
                Ok(SessionInfo {
                    id: id.into(),
                    user_id,
                    valid_until: Utc.timestamp(valid_until as i64, 0),
                })
            },
        )
    }
}
