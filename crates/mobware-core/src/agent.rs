use std::future::Future;
use crate::Message; 

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum AgentError {
    InvalidMessage,
    MessageNotUnderstood,
}

pub trait Agent {
    fn message_received(&self, message: impl Message) -> impl Future<Output=Result<(), AgentError>> + Send + Sync;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestAgent;

    impl Agent for TestAgent {
        fn message_received(&self, message: impl Message) -> impl Future<Output=Result<(), AgentError>> + Send + Sync {
            async move {
                if message.content() == "ValidMessage" {
                    Ok(())
                } else {
                    Err(AgentError::InvalidMessage)
                }
            }
        }
    }

    struct TestMessage(String);
    impl Message for TestMessage {
        fn content(&self) -> &str {
            &self.0
        }
    }

    #[tokio::test]
    async fn test_agent() {
        let agent = TestAgent;
        let message = TestMessage("ValidMessage".to_string());
        let result = agent.message_received(message).await;
        assert_eq!(result, Ok(()));
        let message = TestMessage("InvalidMessage".to_string());
        let result = agent.message_received(message).await;
        assert_eq!(result, Err(AgentError::InvalidMessage));
    }

}