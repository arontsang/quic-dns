use crate::buffer::pooled_buf_mut::PooledBufMut;
use crate::resolver::DnsResolver;
use async_executor::LocalExecutor;
use bytes::Bytes;
use compio::buf::{buf_try, IntoInner, IoBuf};
use compio::io::{AsyncReadExt, AsyncWriteExt};
use compio::net::{TcpListener, TcpStream};
use cyper::Body;
use std::net::SocketAddr;
use std::rc::Rc;
use zeropool::{BufferPool, PooledBuffer};

async fn do_work<T : DnsResolver + 'static>(pool: &Rc<BufferPool>, resolver: Rc<T>, mut client: TcpStream) -> Result<(), std::io::Error>{
    let length = client.read_u16().await?;
    let length: usize = length.into();

    let query = pool.get(length);
    let query = PooledBufMut::new(query);
    let (_, query) = buf_try!(@try client.read_exact(query.slice(..length)).await);

    let query: PooledBuffer = query.into_inner().into();
    let query = Bytes::from_owner(query);
    let query = Body::from(query);
    let response = resolver.resolve(query).await?;

    client.write_u16(response.len() as u16).await?;

    buf_try!(@try client.write_all(response).await);

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