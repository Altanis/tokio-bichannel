# tokio-bichannel

A wrapper around `tokio::sync::mpsc` channels to provide an easy bidirectional stream API.

## How do I install this crate?
Add this to your `Cargo.toml` file:
```
tokio-bichannel = "1"
```

## Examples
```rs
#[tokio::test]
async fn main() {
    let (mut chan1, mut chan2) = channel::<String, String>(10);

    chan1.send("Hello from chan1".to_string()).await.unwrap();
    chan2.send("Hello from chan2".to_string()).await.unwrap();

    let msg1 = chan2.recv().await.unwrap();
    let msg2 = chan1.recv().await.unwrap();

    assert_eq!(msg1, "Hello from chan1");
    assert_eq!(msg2, "Hello from chan2");
}
```