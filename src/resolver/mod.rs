pub(crate) mod quic;

use std::io::Error;
use bytes::Bytes;

pub trait DnsResolver {
    async fn resolve(&'_ self, request: Bytes) -> Result<Bytes, Error>;
}