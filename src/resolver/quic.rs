use crate::resolver::DnsResolver;
use bytes::Bytes;
use cyper::{Body, Client};
use std::io::{Error, ErrorKind};

pub struct QuicResolver {
    client: Client,
    endpoint: String,
}

impl QuicResolver {
    pub async fn new(endpoint: String) -> Self {
        let client = Client::new();
        Self {
            client,
            endpoint,
        }
    }

    async fn resolve_impl(&'_ self, query: Body) -> Result<Bytes, cyper::Error> {
        let http_request = self.client.post(&self.endpoint)?
            .header("accept", "application/dns-message")?
            .header("content-type", "application/dns-message")?;
        let http_request = http_request.body(query);
        let http_response = http_request.send().await?;

        Ok(http_response.bytes().await?)
    }
}

impl DnsResolver for QuicResolver {
    async fn resolve(&'_ self, request: Body) -> Result<Bytes, Error> {
        let result = self.resolve_impl(request).await;
        result.map_err(|err| Error::new(ErrorKind::Other, err))
    }
}