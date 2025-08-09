use chrono::{Utc};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::repositories::{GroupRepository};
use crate::domain;

pub struct LedgerService<R: GroupRepository> {
    repo: Arc<R>,
}

impl<R: GroupRepository> LedgerService<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    pub async fn create_group(&self, name: String) -> Result<domain::groups::GroupId, domain::errors::DomainError> {
        let id = domain::groups::GroupId(Uuid::new_v4());
        let group = domain::groups::Group {
            id: id.clone(),
            name,
            members: vec![],
            expenses: vec![],
        };
        self.repo.save(group).await?;
        Ok(id)
    }

    pub async fn add_member(&self, group_id: domain::groups::GroupId, name: String) -> Result<domain::members::MemberId, domain::errors::DomainError> {
        let mut group = self.repo.get(&group_id).await?;
        let member = domain::members::Member {
            id: domain::members::MemberId(Uuid::new_v4()),
            name,
        };
        let id = member.id.clone();
        group.members.push(member);
        self.repo.save(group).await?;
        Ok(id)
    }

    pub async fn add_expense(
        &self,
        group_id: domain::groups::GroupId,
        paid_by: domain::members::MemberId,
        amount_cents: i64,
        description: String,
        participants: Vec<domain::members::MemberId>,
    ) -> Result<domain::expenses::ExpenseId, domain::errors::DomainError> {
        if amount_cents <= 0 {
            return Err(domain::errors::DomainError::Invalid("amount must be positive".into()));
        }
        let mut group = self.repo.get(&group_id).await?;
        // verify payer is in group
        if !group.members.iter().any(|m| m.id.0 == paid_by.0) {
            return Err(domain::errors::DomainError::Invalid("payer not in group".into()));
        }
        // if participants empty -> all members
        let participants = if participants.is_empty() {
            group.members.iter().map(|m| m.id.clone()).collect()
        } else {
            participants
        };
        // sanity: participants subset of members
        for p in &participants {
            if !group.members.iter().any(|m| m.id.0 == p.0) {
                return Err(domain::errors::DomainError::Invalid("participant not in group".into()));
            }
        }

        let expense = domain::expenses::Expense {
            id: domain::expenses::ExpenseId(Uuid::new_v4()),
            paid_by,
            amount_cents,
            description,
            created_at: Utc::now(),
            participants,
        };
        let id = expense.id.clone();
        group.expenses.push(expense);
        self.repo.save(group).await?;
        Ok(id)
    }

    /// compute net balances for all members in the group
    pub async fn compute_balances(&self, group_id: domain::groups::GroupId) -> Result<Vec<domain::balances::Balance>, domain::errors::DomainError> {
        let group = self.repo.get(&group_id).await?;
        let mut map: HashMap<Uuid, i64> = HashMap::new();
        for m in &group.members {
            map.insert(m.id.0, 0i64);
        }

        for e in &group.expenses {
            let mut participants = e.participants.clone();
            if participants.is_empty() {
                participants = group.members.iter().map(|m| m.id.clone()).collect();
            }
            let n = participants.len() as i64;
            if n == 0 { continue; }
            // split cents evenly; remainder stays with payer (common simple approach)
            let share = e.amount_cents / n;
            // payer receives amount_cents - others' shares
            // simpler: everyone (including payer) owes share; payer effectively paid whole amount, so his net += amount - share
            for p in participants {
                *map.get_mut(&p.0).unwrap() -= share; // each participant owes share
            }
            *map.get_mut(&e.paid_by.0).unwrap() += e.amount_cents; // payer paid whole amount
        }

        let mut balances = vec![];
        for m in &group.members {
            balances.push(domain::balances::Balance {
                member: m.id.clone(),
                balance_cents: *map.get(&m.id.0).unwrap_or(&0),
            })
        }
        Ok(balances)
    }
}