mod address;
mod consts;
mod http_client;
mod http_local;
mod monitor;
mod socks5;
use std::net::SocketAddr;

pub async fn run_tokio(local_listen_addr: SocketAddr, proxy_address: SocketAddr) {
    http_local::run(local_listen_addr, proxy_address)
        .await
        .unwrap()
}