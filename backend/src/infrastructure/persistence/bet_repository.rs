// Se creó un adaptador secundario con implementación postgres 
// del puerto de apuestas

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use crate::domain::{BetTicket, BetStatus, DomainError};
use crate::domain::ports::BetRepository;

pub struct PostgresBetRepository {
    pool: PgPool,
}

impl PostgresBetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BetRepository for PostgresBetRepository {
    async fn save(
        &self,
        id: Uuid,
        ticket: &BetTicket,
        status: &BetStatus,
    ) -> Result<(), DomainError> {
        let status_str = status.as_str();

        sqlx::query!(
            r#"
            INSERT INTO bets (id, user_id, match_id, amount, odds, status, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            id,
            ticket.user_id,
            ticket.match_id,
            ticket.amount,
            ticket.odds,
            status_str,
            Utc::now()
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<BetTicket>, DomainError> {
        let row = sqlx::query!(
            r#"SELECT user_id, match_id, amount, odds FROM bets WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| BetTicket {
            user_id: r.user_id,
            match_id: r.match_id,
            amount: r.amount.map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)).unwrap_or(0.0),
            odds: r.odds.map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)).unwrap_or(0.0),
        }))
    }
}