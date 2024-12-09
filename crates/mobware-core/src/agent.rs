use crate::Message; 

pub trait Agent {
    async fn message_received(&self, message: dyn Message) -> ();
}