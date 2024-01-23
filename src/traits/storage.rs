use std::error::Error;

pub trait Storage<TData, TDataCreate, TDataUpdate> {
    async fn find_all(&mut self) -> Result<Vec<TData>, Box<dyn Error>>;
    async fn create(&mut self, item: TDataCreate) -> Result<(), Box<dyn Error>>;
    async fn update(&mut self, item: TDataUpdate) -> Result<(), Box<dyn Error>>;
    async fn delete_one(&mut self, id: &i32) -> Result<(), Box<dyn Error>>;
    async fn delete_all(&mut self) -> Result<(), Box<dyn Error>>;
}