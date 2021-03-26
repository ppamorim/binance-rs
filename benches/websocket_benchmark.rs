use criterion::{black_box, criterion_group, criterion_main, Criterion};

use binance::api::*;
use binance::userstream::*;
use binance::websockets::*;
use std::sync::atomic::{AtomicBool, Ordering};

fn handle_msg(web_socket: &mut WebSockets<'_>) {
    web_socket.public_handle_msg("test");
}

fn criterion_benchmark(c: &mut Criterion) {

    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let agg_trade: String = String::from("!ticker@arr");
    let mut web_socket: WebSockets<'_> = WebSockets::new(|event: WebsocketEvent| {
        Ok(())
    });

    c.bench_function("handle_msg 20", |b| b.iter(|| handle_msg(&mut web_socket)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);




