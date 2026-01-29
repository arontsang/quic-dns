use crate::resolver::DnsResolver;
use std::net::SocketAddr;
use std::rc::Rc;
use async_executor::LocalExecutor;
use std::io::{Error};
use bytes::Bytes;
use tokio::net::UdpSocket;
use zeropool::BufferPool;
use crate::buffer::sliced_buffer::SlicedBuffer;

async fn do_work<T : DnsResolver + 'static>(query: Bytes, resolver: Rc<T>, socket: Rc<UdpSocket>, client: SocketAddr) -> Result<(), Error>{
    let response = resolver.resolve(query).await?;
    socket.send_to(response.as_ref(), client).await?;
    Ok(())
}

pub async fn start<T : DnsResolver + 'static>(bind_address: SocketAddr, resolver: Rc<T>) -> Result<(), Error> {
    let socket = UdpSocket::bind(bind_address).await?;
    println!("Listening on udp port {}", socket.local_addr()?);
    let socket = Rc::new(socket);

    let local_ex = Rc::new(LocalExecutor::new());
    let pool = BufferPool::new();

    let ret: Result<(), Error> = local_ex.run({
        let socket = socket.clone();
        let local_ex = local_ex.clone();
        async move {
            loop {
                let mut buffer = pool.get(1500);
                let (length, client) =  socket.recv_from(buffer.as_mut_slice()).await?;
                let resolver = resolver.clone();
                let socket = socket.clone();
                local_ex.spawn(async move {
                    let query = buffer;
                    let query = SlicedBuffer::new(query.into(), length);
                    let query = Bytes::from_owner(query);
                    // We don't care about success.
                    // TODO: Log Errors
                    _ = do_work(query, resolver, socket, client).await;
                    ()
                }).detach();
            }
        }
    }).await;
    ret
}