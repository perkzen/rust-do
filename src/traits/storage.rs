use std::error::Error;

pub trait Storage<TData, TDataCreate, TDataUpdate> {
    async fn add(&mut self, item: TDataCreate) -> Result<(), Box<dyn Error>>;
    async fn list(&mut self) -> Result<Vec<TData>, Box<dyn Error>>;
    async fn update(&mut self, item: TDataUpdate) -> Result<(), Box<dyn Error>>;
}