use actix_identity::Identity;
use chrono::Utc;
use tracing::{error, info, instrument};

use super::{user_db::SessionInfo, UserDb};

#[instrument(name = "get_session_from_identity", skip(id, user_db))]
pub async fn get_session_from_identity(id: &Identity, user_db: &UserDb) -> Option<SessionInfo> {
    match id.identity() {
        Some(id) => match user_db.get_session_by_id(&id).await {
            Ok(info) => {
                if Utc::now() > info.valid_until {
                    info!(%info.valid_until, %id, "Session expired");
                    None
                } else {
                    Some(info)
                }
            }
            Err(err) => {
                error!(?err, %id, "Failed to get session");
                None
            }
        },
        None => {
            info!("No authentication found");
            None
        }
    }
}

#[macro_export]
macro_rules! authenticate {
    ( $id:expr, $user_db:expr ) => {{
        let result = $crate::users::get_session_from_identity($id, $user_db).await;
        match result {
            Some(session_info) => session_info,
            None => return Ok(HttpResponse::Forbidden().finish()),
        }
    }};
}
