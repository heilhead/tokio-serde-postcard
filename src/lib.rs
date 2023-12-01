use {
    bytes::{BufMut as _, Bytes, BytesMut},
    serde::{Deserialize, Serialize},
    std::{io, marker::PhantomData, pin::Pin},
    tokio_serde::{Deserializer, Serializer},
};

pub struct Postcard<Item, SinkItem> {
    _marker: PhantomData<(Item, SinkItem)>,
}

impl<Item, SinkItem> Default for Postcard<Item, SinkItem> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Item, SinkItem> Postcard<Item, SinkItem> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

pub type SymmetricalPostcard<T> = Postcard<T, T>;

impl<Item, SinkItem> Deserializer<Item> for Postcard<Item, SinkItem>
where
    for<'a> Item: Deserialize<'a>,
{
    type Error = io::Error;

    fn deserialize(self: Pin<&mut Self>, src: &BytesMut) -> Result<Item, Self::Error> {
        postcard::from_bytes(&src).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
    }
}

impl<Item, SinkItem> Serializer<SinkItem> for Postcard<Item, SinkItem>
where
    SinkItem: Serialize,
{
    type Error = io::Error;

    fn serialize(self: Pin<&mut Self>, data: &SinkItem) -> Result<Bytes, Self::Error> {
        postcard::experimental::serialized_size(data)
            .and_then(|size| postcard::to_io(data, BytesMut::with_capacity(size).writer()))
            .map(|writer| writer.into_inner().freeze())
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
    }
}
