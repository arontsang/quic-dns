pub(crate) mod quic;

use std::io::Error;
use bytes::Bytes;
use cyper::Body;

pub trait DnsResolver {
    async fn resolve(&'_ self, request: Body) -> Result<Bytes, Error>;
}