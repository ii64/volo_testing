#![feature(type_alias_impl_trait)]

use std::{net::SocketAddr, time::Duration, io::Write};
use env_logger::Builder;
use log::LevelFilter;
use the_example::DummyService;
use volo_gen::thrift_gen::dummys::{DummyRequest, DummyServiceClientBuilder};

fn run() {
}

#[volo::main]
async fn main() {
    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    let bff = backoff::ExponentialBackoffBuilder::new()
        .build();

    let cf = DummyServiceClientBuilder::new("dummy-service")
        .address(addr.clone())
        .build();

    tokio::spawn(async move {
        log::info!("running client");
        loop {
            match backoff::future::retry_notify(bff.clone(), || async {
                let mut client = cf.clone();
                Ok(client.dummy_method(DummyRequest { id: Some("123123123123".into()) }).await?)
            }, |err, dur| println!("retry err {:?}: {:?}", err, dur)).await {
                Ok(a) => log::info!("OK {:?}", a),
                Err(e) => log::error!("ERR {:?}", e),
            };
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    });

    log::info!("running server...");
    volo_gen::thrift_gen::dummys::DummyServiceServer::new(DummyService)
        .run(addr)
        .await
        .unwrap();
}
