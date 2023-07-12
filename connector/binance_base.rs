use binance::futures::websockets::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;
use serde_json::{Result, Value};

extern {
    fn rustMessageHandler(name: *const c_char);
}

#[link(name = "callcpp", kind = "dylib")]
extern "C" {}

#[no_mangle]
pub extern "C"  

fn cppMessageHandler(msg: *const c_char) {

    unsafe {
        let c_str_msg = CStr::from_ptr(msg);
        let rust_str_msg = c_str_msg.to_str().unwrap();
        
        let rust_args: Value = serde_json::from_str(rust_str_msg).unwrap();

        if rust_args["cmd"] == "new_order" {
            
        } else if rust_args["cmd"] == "ws_start" {

        } else if rust_args["cmd"] == "batch_new_order" {

        } else if rust_args["cmd"] == "cancel_all_orders" {

        } else if rust_args["cmd"] == "exchange_info" {

        }
    }
}

fn market_websocket() {
    // Example to show the future market websockets. It will print one event for each
    // endpoint and continue to the next.

    let keep_running = AtomicBool::new(true);
    let stream_examples_usd_m = vec![
        // taken from https://binance-docs.github.io/apidocs/futures/en/#websocket-market-streams
        "btcusdt@depth@0ms"
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
            rustMessageHandler(name_ptr)
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
