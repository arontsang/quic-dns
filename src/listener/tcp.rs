use crate::resolver::DnsResolver;
use async_executor::LocalExecutor;
use bytes::Bytes;

use std::net::SocketAddr;
use std::rc::Rc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use zeropool::{BufferPool};

async fn do_work<T : DnsResolver + 'static>(pool: &Rc<BufferPool>, resolver: Rc<T>, mut client: TcpStream) -> Result<(), std::io::Error>{
    let length = client.read_u16().await?;
    let length: usize = length.into();

    let mut query = pool.get(length);


    client.read_exact(&mut query.as_mut_slice()[..length]).await?;


    let query = Bytes::from_owner(query);
    let response = resolver.resolve(query).await?;

    client.write_u16(response.len() as u16).await?;
    client.write_all(&response).await?;

    Ok(())
}

pub async fn start<T : DnsResolver + 'static >(bind_address: SocketAddr, resolver: Rc<T>) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(bind_address).await?;
    println!("Listening on tcp port {}", listener.local_addr()?);
    let local_ex = Rc::new(LocalExecutor::new());
    local_ex.run(async {
        let local_ex = local_ex.clone();
        let pool = Rc::new(BufferPool::new());
        loop {

            if let Ok((socket, _)) = listener.accept().await {
                local_ex.spawn({
                    let resolver = resolver.clone();
                    let mut pool = pool.clone();
                    async move {
                        // We don't care about success.
                        // TODO: Log Errors
                        let _  = do_work(&mut pool, resolver, socket).await;
                    }
                }).detach();
            }
        }
    }).await;
    Ok(())
}