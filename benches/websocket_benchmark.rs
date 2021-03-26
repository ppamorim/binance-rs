use criterion::{black_box, criterion_group, criterion_main, Criterion};

use binance::api::*;
use binance::userstream::*;
use binance::websockets::*;
use std::sync::atomic::{AtomicBool, Ordering};
use core::time::Duration;

fn handle_msg(web_socket: &mut WebSockets<'_>, body: &str) {
    web_socket.public_handle_msg(&body);
}

fn criterion_benchmark(c: &mut Criterion) {

    let mut group = c.benchmark_group("websockets-decoder");

    let body = reqwest::blocking::get("https://api.binance.com/api/v3/ticker/price")
        .unwrap().text().unwrap();

    println!("Length: {}", body.len());

    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let agg_trade: String = String::from("!ticker@arr");
    let mut web_socket: WebSockets<'_> = WebSockets::new(|event: WebsocketEvent| {
        Ok(())
    });

    group.sample_size(200);
    group.measurement_time(Duration::new(35, 0));
    group.bench_function("handle_msg 20", |b| b.iter(|| handle_msg(&mut web_socket, &body)));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);