# tokio-serde-postcard
[`tokio-serde`](https://github.com/carllerche/tokio-serde) transport based on [`postcard`](https://github.com/jamesmunns/postcard) format.

## Usage

Adding dependency:

```toml
[dependencies]
tokio-serde-postcard = "0.1"
```

Transport initialization is the same as the built-in `tokio_serde` serializers:

```rust
fn framed_tcp_stream<Item, SinkItem>(
    stream: TcpStream,
) -> tokio_serde::Framed<
    tokio_util::codec::Framed<TcpStream, LengthDelimitedCodec>,
    Item,
    SinkItem,
    tokio_serde_postcard::Postcard<Item, SinkItem>,
> {
    // First wrap the stream with a basic length-delimited codec.
    let transport =
        tokio_util::codec::Framed::new(stream, tokio_util::codec::LengthDelimitedCodec::default());

    // Then wrap the transport with `tokio_serde`.
    tokio_serde::Framed::new(transport, tokio_serde_postcard::Postcard::default())
}
```

# License

[Apache 2.0](LICENSE)
