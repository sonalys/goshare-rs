use crate::domain;

#[async_trait::async_trait]
pub trait GroupRepository: Send + Sync {
    async fn save(&self, group: domain::groups::Group) -> Result<(), domain::errors::DomainError>;
    async fn get(&self, id: &domain::groups::GroupId) -> Result<domain::groups::Group, domain::errors::DomainError>;
}