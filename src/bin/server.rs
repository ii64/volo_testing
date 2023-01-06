#![feature(type_alias_impl_trait)]

use std::net::SocketAddr;

use volo_gen::{S};

#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    volo_gen::dummys::DummyServiceServer::new(S)
        .run(addr)
        .await
        .unwrap();
}
