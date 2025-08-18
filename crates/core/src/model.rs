use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tokio_stream::wrappers::ReceiverStream;

/// Trait for model adapters that stream tokens for a given message.
pub trait Model: Send + Sync {
    /// Start generating a reply to `message`.
    ///
    /// Implementations return a stream of tokens which can be consumed
    /// asynchronously to display incremental model output.
    fn chat(&self, message: String) -> ReceiverStream<String>;
}

/// Simple model adapter that echoes the user's message back token by token.
#[derive(Default)]
pub struct EchoModel;

impl Model for EchoModel {
    fn chat(&self, message: String) -> ReceiverStream<String> {
        let (tx, rx) = mpsc::channel(32);
        tokio::spawn(async move {
            for ch in message.chars() {
                if tx.send(ch.to_string()).await.is_err() {
                    break;
                }
                sleep(Duration::from_millis(50)).await;
            }
        });
        ReceiverStream::new(rx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_stream::StreamExt;

    #[tokio::test]
    async fn echo_model_streams() {
        let model = EchoModel::default();
        let stream = model.chat("hi".to_string());
        let collected: Vec<String> = stream.collect().await;
        assert_eq!(collected.join(""), "hi");
    }
}
