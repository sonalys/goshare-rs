use std::collections::HashMap;

use uuid::Uuid;

use crate::{domain, repositories};

pub struct InMemoryGroupRepo {
    // using parking_lot mutex for simplicity & speed
    inner: parking_lot::RwLock<HashMap<Uuid,domain::groups::Group>>,
}

impl InMemoryGroupRepo {
    pub fn new() -> Self {
        Self {
            inner: parking_lot::RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl repositories::GroupRepository for InMemoryGroupRepo {
    async fn save(&self, group: domain::groups::Group) -> Result<(), domain::errors::DomainError> {
        self.inner.write().insert((group.id).0, group);
        Ok(())
    }

    async fn get(&self, id: &domain::groups::GroupId) -> Result<domain::groups::Group, domain::errors::DomainError> {
        match self.inner.read().get(&id.0) {
            Some(g) => Ok(g.clone()),
            None => Err(domain::errors::DomainError::NotFound(format!("group {}", id.0))),
        }
    }
}