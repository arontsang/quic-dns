use crate::resolver::DnsResolver;
use compio::net::UdpSocket;
use std::net::SocketAddr;
use std::rc::Rc;
use async_executor::LocalExecutor;
use std::io::Error;
use compio::buf::buf_try;
use cyper::Body;

async fn do_work<T : DnsResolver + 'static>(query: &[u8], resolver: Rc<T>, socket: Rc<UdpSocket>, client: SocketAddr) -> Result<(), Error>{
    let body = query.to_vec();
    let body = Body::from(body);
    let response = resolver.resolve(body).await?;
    buf_try!(@try socket.send_to(response, client).await);
    Ok(())
}

pub async fn start<T : DnsResolver + 'static>(bind_address: SocketAddr, resolver: Rc<T>) -> Result<(), Error> {
    let socket = UdpSocket::bind(bind_address).await?;
    let socket = Rc::new(socket);

    let local_ex = Rc::new(LocalExecutor::new());

    let ret: Result<(), Error> = local_ex.run({
        let socket = socket.clone();
        let local_ex = local_ex.clone();
        async move {
            loop {
                let buffer: Vec<u8> = Vec::with_capacity(1500);
                let ((length, client), buffer) =  buf_try!(@try socket.recv_from(buffer).await);
                let resolver = resolver.clone();
                let socket = socket.clone();
                local_ex.spawn(async move {


                    let query = &buffer.as_slice()[..length];
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