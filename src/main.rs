#![cfg_attr(feature = "strict", deny(warnings))]
mod resolver;
mod listener;
mod buffer;

use std::net::SocketAddr;
use std::rc::Rc;
use async_executor::LocalExecutor;
use rustop::opts;

fn main() -> Result<(), Box<std::io::Error>> {
    let runtime = compio::runtime::Runtime::new()?;
    runtime.block_on(main_async());
    Ok(())
}

async fn main_async() {
    let (args, _) = opts! {
        synopsis "This is a DNS stub server that proxies to a DOH server.";
        opt port:Option<u16>, desc:"Port to host DNS proxy Defaults to: 15353";
        opt server:Option<String>, desc:"DOH Server defaults to: https://1.1.1.1/dns-query" ;
    }.parse_or_exit();

    let port = args.port.unwrap_or(15353);
    let server = args.server.unwrap_or("https://1.1.1.1/dns-query".to_string());

    let resolver = resolver::quic::QuicResolver::new(server).await;
    let resolver = Rc::new(resolver);

    let local_ex = LocalExecutor::new();


    let tcp = local_ex.spawn(async {
        let resolver = resolver.clone();
        async move {
            let binding_socket = SocketAddr::from(([0, 0, 0, 0], port));
            listener::tcp::start(binding_socket, resolver).await
        }.await
    });

    let udp = local_ex.spawn(async {
        let resolver = resolver.clone();
        async move {
            let binding_socket = SocketAddr::from(([0, 0, 0, 0], port));
            listener::udp::start(binding_socket, resolver).await
        }.await
    });

    _ = local_ex.run(async move {
        _ = tcp.await;
        _ = udp.await;
    }).await;
}
