use crate::resolver::DnsResolver;
use async_executor::LocalExecutor;
use compio::buf::buf_try;
use compio::io::{AsyncReadExt, AsyncWriteExt};
use compio::net::{TcpListener, TcpStream};
use std::net::SocketAddr;
use std::rc::Rc;
use cyper::Body;

async fn do_work<T : DnsResolver + 'static>(resolver: Rc<T>, mut client: TcpStream) -> Result<(), std::io::Error>{
    let length = client.read_u16().await?;
    let length: usize = length.into();

    let query = Vec::with_capacity(length);
    let (_, query) = buf_try!(@try client.read_exact(query).await);

    let query = query.to_vec();
    let query = Body::from(query);
    let response = resolver.resolve(query).await?;

    client.write_u16(response.len() as u16).await?;

    buf_try!(@try client.write_all(response).await);

    Ok(())
}

pub async fn start<T : DnsResolver + 'static >(bind_address: SocketAddr, resolver: Rc<T>) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(bind_address).await?;
    let local_ex = Rc::new(LocalExecutor::new());
    local_ex.run(async {
        let local_ex = local_ex.clone();
        loop {

            if let Ok((socket, _)) = listener.accept().await {
                local_ex.spawn({
                    let resolver = resolver.clone();
                    async move {
                        // We don't care about success.
                        // TODO: Log Errors
                        let _  = do_work(resolver, socket).await;
                    }
                }).detach();
            }
        }
    }).await;
    Ok(())
}