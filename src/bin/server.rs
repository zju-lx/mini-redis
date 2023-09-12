#![feature(impl_trait_in_assoc_type)]
use mini_redis::FilterLayer;
use std::net::SocketAddr;

use mini_redis::{S};

#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    volo_gen::volo::example::RedisServiceServer::new(S)
        .layer_front(FilterLayer)
        .run(addr)
        .await
        .unwrap();
}
