pub trait Message {
    fn content(&self) -> String;
}