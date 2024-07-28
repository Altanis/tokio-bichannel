use tokio::sync::mpsc::{channel as create_channel, Receiver, Sender};
use tokio::sync::mpsc::error::{SendError, TryRecvError};

/// A bidirectional channel structure that supports sending and receiving messages.
/// 
/// # Examples
/// 
/// ```
/// let (mut l, mut r) = channel::<String, String>(10);
/// 
/// l.send("Hello from chan1".to_string()).await.unwrap();
/// r.send("Hello from chan2".to_string()).await.unwrap();
/// 
/// assert_eq!(r.recv().await.unwrap(), "Hello from chan1");
/// assert_eq!(l.recv().await.unwrap(), "Hello from chan2");
/// ```
#[derive(Debug)]
pub struct Channel<S, R> {
    sender: Sender<S>,
    receiver: Receiver<R>,
}

impl<S, R> Channel<S, R> {
    /// Sends a message through the channel.
    /// 
    /// # Arguments
    /// 
    /// * `s` - The message to send.
    /// 
    /// # Returns
    /// 
    /// * `Result<(), SendError<S>>` - Returns `Ok(())` if the message was sent successfully, or an error if it wasn't.
    /// 
    /// # Examples
    /// 
    /// ```
    /// channel.send("Hello".to_string()).await.unwrap();
    /// ```
    pub async fn send(&self, s: S) -> Result<(), SendError<S>> {
        self.sender.send(s).await
    }

    /// Receives a message from the channel.
    /// 
    /// # Returns
    /// 
    /// * `Option<R>` - Returns `Some(message)` if a message was received, or `None` if the channel is closed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// while let Some(msg) = channel.recv().await {
    ///     println!("Received: {}", msg);
    /// }
    /// ```
    pub async fn recv(&mut self) -> Option<R> {
        self.receiver.recv().await
    }

    /// Attempts to receive a message from the channel without blocking.
    /// 
    /// # Returns
    /// 
    /// * `Result<R, TryRecvError>` - Returns `Ok(message)` if a message was received, or an error if the channel is empty or closed.
    /// 
    /// # Examples
    /// ```
    /// match channel.try_recv() {
    ///     Ok(msg) => println!("Received: {}", msg),
    ///     Err(e) => println!("Error: {:?}", e),
    /// }
    /// ```
    pub fn try_recv(&mut self) -> Result<R, TryRecvError> {
        self.receiver.try_recv()
    }
}

/// Creates a bidirectional channel with the specified buffer size.
/// 
/// # Arguments
/// 
/// * `buffer_size` - The size of the buffer for the channel.
/// 
/// # Returns
/// 
/// * `(Channel<T, U>, Channel<U, T>)` - Returns a tuple of two `Channel` instances.
/// 
/// # Examples
/// 
/// ```
/// use tokio_bichannel::{channel, Channel};
/// 
/// #[tokio::main]
/// async fn main() {
///     let (mut chan1, mut chan2) = channel::<String, String>(10);
/// 
///     chan1.send("Hello from chan1".to_string()).await.unwrap();
///     chan2.send("Hello from chan2".to_string()).await.unwrap();
/// 
///     let msg1 = chan2.recv().await.unwrap();
///     let msg2 = chan1.recv().await.unwrap();
/// 
///     assert_eq!(msg1, "Hello from chan1");
///     assert_eq!(msg2, "Hello from chan2");
/// }
/// ```
pub fn channel<T, U>(buffer_size: usize) -> (Channel<T, U>, Channel<U, T>) {
    let (ls, lr) = create_channel(buffer_size);
    let (rs, rr) = create_channel(buffer_size);

    (
        Channel {
            sender: ls,
            receiver: rr,
        },
        Channel {
            sender: rs,
            receiver: lr,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;
    use std::thread;

    #[tokio::test]
    async fn test_send_recv() {
        let (mut chan1, mut chan2) = channel::<String, String>(10);

        chan1.send("Hello from chan1".to_string()).await.unwrap();
        chan2.send("Hello from chan2".to_string()).await.unwrap();

        let msg1 = chan2.recv().await.unwrap();
        let msg2 = chan1.recv().await.unwrap();

        assert_eq!(msg1, "Hello from chan1");
        assert_eq!(msg2, "Hello from chan2");
    }

    #[tokio::test]
    async fn test_try_recv() {
        let (mut chan1, mut chan2) = channel::<String, String>(10);

        chan1.send("Hello from chan1".to_string()).await.unwrap();

        let msg1 = chan2.try_recv().unwrap();
        assert_eq!(msg1, "Hello from chan1");

        let try_recv_result = chan2.try_recv();
        assert!(try_recv_result.is_err());
    }

    #[tokio::test]
    async fn test_threading() {
        let (mut chan1, mut chan2) = channel::<String, String>(10);

        let handle = thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                chan1.send("Hello from chan1".to_string()).await.unwrap();
                let msg = chan1.recv().await.unwrap();
                assert_eq!(msg, "Hello from chan2");
            });
        });

        chan2.send("Hello from chan2".to_string()).await.unwrap();
        let msg = chan2.recv().await.unwrap();
        assert_eq!(msg, "Hello from chan1");

        handle.join().unwrap();
    }
}
