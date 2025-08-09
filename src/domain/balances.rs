use serde::{Deserialize, Serialize};

use crate::domain;

// Value object for balances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub member: domain::members::MemberId,
    /// positive means others owe this member; negative means this member owes others
    pub balance_cents: i64,
}