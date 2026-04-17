use async_trait::async_trait;

#[async_trait]
pub trait DataExternalStore {
    async fn save();
}
