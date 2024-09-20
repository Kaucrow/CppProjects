use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ConfirmationToken {
    pub user_id: uuid::Uuid,
}