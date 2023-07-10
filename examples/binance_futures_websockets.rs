use binance::futures::websockets::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use std::ffi::CString;
use std::os::raw::c_char;

fn main() {
    market_websocket();
}
extern {
    fn callApiEndpointFromRust(name: *const c_char);
}

#[link(name = "callcpp", kind = "dylib")]
extern "C" {}

fn market_websocket() {
    // Example to show the future market websockets. It will print one event for each
    // endpoint and continue to the next.

    let keep_running = AtomicBool::new(true);
    let stream_examples_usd_m = vec![
        // taken from https://binance-docs.github.io/apidocs/futures/en/#websocket-market-streams
        "btcusdt@depth@0ms",                     // <symbol>@aggTrade
    ];

    let callback_fn = |event: FuturesWebsocketEvent| {
        // once a FuturesWebsocketEvent is recevied, we print it
        // and stop this socket, so the example will continue to the next one
        //
        // in case an event comes in that doesn't properly serialize to
        // a FuturesWebsocketEvent, the web socket loop will keep running
        // println!("{:?}\n", event);
        let name = CString::new(format!("{:?}", event)).unwrap();
        let name_ptr = name.as_ptr();
        unsafe {
            callApiEndpointFromRust(name_ptr)
        }
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let in_ns = since_the_epoch.as_secs() as u64 * 1_000_000_000 +
            since_the_epoch.subsec_nanos() as u64;
        println!("{}\n", in_ns);

        Ok(())
    };

    // USD-M futures examples
    for stream_example in stream_examples_usd_m {
        println!("Starting with USD_M {:?}", stream_example);
        keep_running.swap(true, Ordering::Relaxed);

        let mut web_socket: FuturesWebSockets<'_> = FuturesWebSockets::new(callback_fn);
        web_socket
            .connect(&FuturesMarket::USDM, stream_example)
            .unwrap();
        web_socket.event_loop(&keep_running).unwrap();
        web_socket.disconnect().unwrap();
    }
}
