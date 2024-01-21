use std::error::Error;


pub trait Storage<TData, TDataCreate> {
    async fn add(&mut self, item: TDataCreate) -> Result<(), Box<dyn Error>>;
    async fn list(&mut self) -> Result<Vec<TData>, Box<dyn Error>>;
}