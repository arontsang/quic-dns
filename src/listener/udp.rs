use crate::resolver::DnsResolver;
use compio::net::UdpSocket;
use std::net::SocketAddr;
use std::rc::Rc;
use async_executor::LocalExecutor;
use std::io::Error;
use bytes::Bytes;
use compio::buf::buf_try;
use cyper::Body;
use zeropool::BufferPool;
use crate::buffer::pooled_buf_mut::PooledBufMut;
use crate::buffer::sliced_buffer::SlicedBuffer;

async fn do_work<T : DnsResolver + 'static>(query: Body, resolver: Rc<T>, socket: Rc<UdpSocket>, client: SocketAddr) -> Result<(), Error>{
    let response = resolver.resolve(query).await?;
    buf_try!(@try socket.send_to(response, client).await);
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
                let buffer = pool.get(1500);
                let buffer = PooledBufMut::new(buffer);
                let ((length, client), buffer) =  buf_try!(@try socket.recv_from(buffer).await);
                let resolver = resolver.clone();
                let socket = socket.clone();
                local_ex.spawn(async move {


                    let query = buffer;
                    let query = SlicedBuffer::new(query.into(), length);
                    let query = Bytes::from_owner(query);
                    let query = Body::from(query);
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