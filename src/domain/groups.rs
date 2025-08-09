use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::{members,expenses};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GroupId(pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: GroupId,
    pub name: String,
    pub members: Vec<members::Member>,
    pub expenses: Vec<expenses::Expense>,
}