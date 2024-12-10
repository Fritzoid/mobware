pub trait Message: Send + Sync {
    fn content(&self) -> &str;
}