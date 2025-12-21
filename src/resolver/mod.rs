pub(crate) mod quic;

use std::io::Error;
use bytes::Bytes;

pub trait DnsResolver {
    async fn resolve(&'_ self, request: &[u8]) -> Result<Bytes, Error>;
}