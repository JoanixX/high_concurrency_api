// entidades de dominio puras sin dtos de http
// los request/response types van en el adaptador de handlers

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BetTicket {
    pub user_id: Uuid,
    pub match_id: Uuid,
    pub amount: f64,
    pub odds: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BetStatus {
    Pending,
    Validated,
    Rejected,
}

impl BetStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            BetStatus::Pending => "PENDING",
            BetStatus::Validated => "VALIDATED",
            BetStatus::Rejected => "REJECTED",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
}