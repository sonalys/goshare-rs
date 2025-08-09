use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::{members};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExpenseId(pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    pub id: ExpenseId,
    pub paid_by: members::MemberId,
    pub amount_cents: i64,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub participants: Vec<members::MemberId>,
}