use std::borrow::Borrow;
use std::ffi::{CStr, CString};
// use crate::futures::websockets::*;
use std::os::raw::{c_char, c_void};
// use serde_json::Value;
use crate::futures::account::*;
use crate::api::Binance;
use crate::futures::general::FuturesGeneral;
use crate::futures::websockets::*;
use crate::futures::userstream::*;
use crate::errors::ErrorKind as BinanceLibErrorKind;
use std::sync::atomic::AtomicBool;
use std::thread;
use std::time::Duration;

extern fn dummy(_: *const c_char) -> *mut c_char {
    std::ptr::null_mut()
}

static mut FUNC_CPP_FROM_RUST: extern fn(s: *const c_char) -> *mut c_char = dummy;
static mut API_KEY: Option<String> = None;
static mut SECRET_KEY: Option<String> = None;
static mut ACCOUNT: Option<FuturesAccount> = None;
static mut GENERAL: Option<FuturesGeneral> = None;
static mut MARKET: Option<crate::futures::market::FuturesMarket> = None;
static mut STREAMS: Vec<String> = vec![];
static mut USER: Option<FuturesUserStream> = None;

///
/// Must be called at beginning
/// 
pub fn init(api: *const c_char, secret: *const c_char) {
    unsafe {
        let api_rs = CStr::from_ptr(api).to_str().unwrap();
        let secret_rs = CStr::from_ptr(secret).to_str().unwrap();
        API_KEY.get_or_insert_with(|| api_rs.to_owned());
        SECRET_KEY.get_or_insert_with(|| secret_rs.to_owned());
        ACCOUNT.get_or_insert(Binance::new(API_KEY.clone(), SECRET_KEY.clone()));
        GENERAL.get_or_insert(Binance::new(API_KEY.clone(), SECRET_KEY.clone()));
        MARKET.get_or_insert(Binance::new(API_KEY.clone(), SECRET_KEY.clone()));
        USER.get_or_insert(Binance::new(API_KEY.clone(), SECRET_KEY.clone()));
    }
}

#[no_mangle]
pub extern "C" fn init_from_cpp(api: *const c_char, secret: *const c_char) -> i32 {
    init(api, secret);
    0
}

#[no_mangle]
pub extern "C" fn ws_order_book_rs(symbol: *const c_char) -> i32 {
    let rs_symbol: String;
    unsafe {
        rs_symbol = CStr::from_ptr(symbol).to_str().unwrap().to_owned() + "@depth@0ms";
        STREAMS.push(rs_symbol);
    }
    0
}

#[no_mangle]
pub extern "C" fn ws_agg_trade_rs(symbol: *const c_char) -> i32 {
    let rs_symbol: String;
    unsafe {
        rs_symbol = CStr::from_ptr(symbol).to_str().unwrap().to_owned() + "@aggTrade";
        STREAMS.push(rs_symbol);
    }
    0
}

#[no_mangle]
pub extern "C" fn ws_mark_price_rs(symbol: *const c_char) -> i32 {
    let rs_symbol: String;
    unsafe {
        rs_symbol = CStr::from_ptr(symbol).to_str().unwrap().to_owned() + "@markPrice";
        STREAMS.push(rs_symbol);
    }
    0
}

#[no_mangle]
pub extern "C" fn ws_start(data: *mut c_void, callback: extern fn(_: *mut c_char, __: *mut c_void) -> *mut c_char) -> i32 {
    let callback_fn = |msg: &str| {
        callback(CString::new(format!("{}", msg)).unwrap().into_raw() as *mut c_char, data);
        Ok(())
    };
    unsafe {
        for s in STREAMS.clone() {
            println!("{}", s);
        }
    }
    let streams: &[String];
    unsafe {
        streams = STREAMS.borrow();
    }
    loop {
        let keep_running = AtomicBool::new(true);
        let mut web_socket: FuturesWebSockets<'_> = FuturesWebSockets::new(callback_fn);
        web_socket
            .connect_multiple_streams(&FuturesMarket::USDM, streams)
            .unwrap();
        if let Err(e) = web_socket.event_loop(&keep_running) {
            println!("{}", e);
            web_socket.disconnect().unwrap();
            continue;
        }
    }
}

#[no_mangle]
pub extern "C" fn ws_user_data_rs() -> i32 {
    unsafe {
        if let Ok(answer) = USER.as_mut().unwrap().start() {
            let listen_key = answer.listen_key;
            let listen_key_clone = listen_key.clone();

            STREAMS.push(listen_key.clone());

            thread::spawn(move || {
            loop {
                    thread::sleep(Duration::from_secs(1800));
                    match USER.as_mut().unwrap().keep_alive(&listen_key_clone) {
                        Ok(_msg) => continue,
                        Err(_e) => break,
                    }
                }
            });
            return 0;
        } else {
            println!("Not able to start an User Stream (Check your API_KEY)");
            return 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn rust_from_cpp(s: *const c_char) -> *mut c_char {
    unsafe {
        // let c_str = CStr::from_ptr(s);
        // let rust_str = c_str.to_str().unwrap();
    
        FUNC_CPP_FROM_RUST(s);
        std::ptr::null_mut()
    }
}

#[no_mangle]
//pub fn cancel_order_with_client_id<S>(&self, symbol: S, orig_client_order_id: String) 
pub extern "C" fn cancel_order_with_client_id_rs(symbol: *const c_char, orig_client_order_id: *const c_char) -> *mut c_char {
    unsafe {
        let rs_symbol = CStr::from_ptr(symbol).to_str().unwrap();
        let cstr = CStr::from_ptr(orig_client_order_id);
        let str_slice = cstr.to_str().expect("Invalid UTF-8 string");
        let rs_orig_client_order_id: String = String::from(str_slice);
        
        let res = match ACCOUNT.as_mut().unwrap().cancel_order_with_client_id(rs_symbol, rs_orig_client_order_id) {
            Ok(answer) => format!("{:?}", answer),
            Err(e) => {
                match e.0 {
                    BinanceLibErrorKind::BinanceError(response) => format!("{{ec: \"{}\", errmsg: \"{}\"}}", response.code, response.msg),
                    BinanceLibErrorKind::Msg(msg) => format!("{{ec: 1, errmsg: \"{}\"}}", msg),
                    _ => format!("{{ec: 1, errmsg: \"{}\"}}", e.0),
                }
            },
        };

        CString::new(res).unwrap().into_raw() as *mut c_char
    }
}

#[no_mangle]
// pub fn cancel_order<S>(&self, symbol: S, order_id: u64) -> Result<OrderCanceled>
pub extern "C" fn cancel_order_rs(symbol: *const c_char, order_id: *const c_char) -> *mut c_char {
    unsafe {
        let rs_symbol = CStr::from_ptr(symbol).to_str().unwrap();
        let rs_order_id: &str = CStr::from_ptr(order_id).to_str().unwrap();
        
        let res = match ACCOUNT.as_mut().unwrap().cancel_order(rs_symbol, rs_order_id.parse::<u64>().unwrap()) {
            Ok(answer) => format!("{:?}", answer),
            Err(e) => {
                match e.0 {
                    BinanceLibErrorKind::BinanceError(response) => format!("{{ec: \"{}\", errmsg: \"{}\"}}", response.code, response.msg),
                    BinanceLibErrorKind::Msg(msg) => format!("{{ec: 1, errmsg: \"{}\"}}", msg),
                    _ => format!("{{ec: 1, errmsg: \"{}\"}}", e.0),
                }
            },
        };

        CString::new(res).unwrap().into_raw() as *mut c_char
    }
}

#[no_mangle]
// pub fn custom_order<S, F>(&self, symbol: S, qty: F, price: f64, stop_price: Option<f64>, order_side: OrderSide,
//    order_type: OrderType, time_in_force: TimeInForce, new_client_order_id: Option<String>, ) -> Result<Transaction>
pub extern "C" fn custom_order_rs(request: *const c_char) -> *mut c_char {
    unsafe {
        let order = CStr::from_ptr(request).to_str().unwrap();
        let ores = ACCOUNT.as_mut().unwrap().custom_order_fast(order);
        let res = match ores {
            Ok(answer) => answer,
            Err(e) => {
                match e.0 {
                    BinanceLibErrorKind::BinanceError(response) => format!("{{ec: \"{}\", errmsg: \"{}\"}}", response.code, response.msg),
                    BinanceLibErrorKind::Msg(msg) => format!("{{ec: 1, errmsg: \"{}\"}}", msg),
                    _ => format!("{{ec: 1, errmsg: \"{}\"}}", e.0),
                }
            },
        };
        CString::new(res).unwrap().into_raw() as *mut c_char
    }
}


#[no_mangle]
// pub fn exchange_info(&self) -> Result<ExchangeInformation>
pub extern "C" fn exchange_info_rs() -> *mut c_char {
    unsafe {
        let res = match GENERAL.as_mut().unwrap().exchange_info() {
            Ok(answer) => format!("{:?}", answer),
            Err(e) => {
                match e.0 {
                    BinanceLibErrorKind::BinanceError(response) => format!("{{ec: \"{}\", errmsg: \"{}\"}}", response.code, response.msg),
                    BinanceLibErrorKind::Msg(msg) => format!("{{ec: 1, errmsg: \"{}\"}}", msg),
                    _ => format!("{{ec: 1, errmsg: \"{}\"}}", e.0),
                }
            },
        };
        CString::new(res).unwrap().into_raw() as *mut c_char
    }
}

#[no_mangle]
// pub fn account_balance(&self) -> Result<ExchangeInformation>
pub extern "C" fn account_balance_rs() -> *mut c_char {
    unsafe {
        let res = match ACCOUNT.as_mut().unwrap().account_balance() {
            Ok(answer) => format!("{:?}", answer),
            Err(e) => {
                match e.0 {
                    BinanceLibErrorKind::BinanceError(response) => format!("{{ec: \"{}\", errmsg: \"{}\"}}", response.code, response.msg),
                    BinanceLibErrorKind::Msg(msg) => format!("{{ec: 1, errmsg: \"{}\"}}", msg),
                    _ => format!("{{ec: 1, errmsg: \"{}\"}}", e.0),
                }
            },
        };

        CString::new(res).unwrap().into_raw() as *mut c_char
    }
}

#[no_mangle]
// pub fn exchange_info(&self) -> Result<ExchangeInformation>
pub extern "C" fn cancel_all_open_orders_rs(symbol: *const c_char) -> *mut c_char {
    unsafe {
        let rs_symbol = CStr::from_ptr(symbol).to_str().unwrap();

        let res = match ACCOUNT.as_mut().unwrap().cancel_all_open_orders(rs_symbol) {
            Ok(answer) => format!("{{\"data\":[{:?}]}}", answer),
            Err(e) => {
                match e.0 {
                    BinanceLibErrorKind::BinanceError(response) => format!("{{ec: \"{}\", errmsg: \"{}\"}}", response.code, response.msg),
                    BinanceLibErrorKind::Msg(msg) => format!("{{ec: 1, errmsg: \"{}\"}}", msg),
                    _ => format!("{{ec: 1, errmsg: \"{}\"}}", e.0),
                }
            },
        };

        CString::new(res).unwrap().into_raw() as *mut c_char
    }
}

#[no_mangle]
// pub fn get_custom_depth<S>(&self, symbol: S, depth: u64) -> Result<OrderBook>
pub extern "C" fn get_custom_depth_rs(symbol: *const c_char, depth: *const c_char) -> *mut c_char {    
    unsafe {
        let rs_symbol = CStr::from_ptr(symbol).to_str().unwrap();
        let rs_depth = CStr::from_ptr(depth).to_str().unwrap();

        let res = match MARKET.as_mut().unwrap().get_custom_depth(rs_symbol, rs_depth.parse::<u64>().unwrap()) {
            Ok(answer) => format!("{:?}", answer),
            Err(e) => {
                match e.0 {
                    BinanceLibErrorKind::BinanceError(response) => format!("{{ec: \"{}\", errmsg: \"{}\"}}", response.code, response.msg),
                    BinanceLibErrorKind::Msg(msg) => format!("{{ec: 1, errmsg: \"{}\"}}", msg),
                    _ => format!("{{ec: 1, errmsg: \"{}\"}}", e.0),
                }
            },
        };

        CString::new(res).unwrap().into_raw() as *mut c_char
    }
}

#[no_mangle]
// pub fn get_price<S>(&self, symbol: S) -> Result<SymbolPrice>
pub extern "C" fn get_price_rs(symbol: *const c_char) -> *mut c_char {
    unsafe {
        let rs_symbol = CStr::from_ptr(symbol).to_str().unwrap();

        let res = match MARKET.as_mut().unwrap().get_price(rs_symbol) {
            Ok(answer) => format!("{:?}", answer),
            Err(e) => {
                match e.0 {
                    BinanceLibErrorKind::BinanceError(response) => format!("{{ec: \"{}\", errmsg: \"{}\"}}", response.code, response.msg),
                    BinanceLibErrorKind::Msg(msg) => format!("{{ec: 1, errmsg: \"{}\"}}", msg),
                    _ => format!("{{ec: 1, errmsg: \"{}\"}}", e.0),
                }
            },
        };

        CString::new(res).unwrap().into_raw() as *mut c_char
    }
}

#[no_mangle]
// pub fn get_book_ticker<S>(&self, symbol: S) -> Result<Tickers>
pub extern "C" fn get_book_ticker_rs(symbol: *const c_char) -> *mut c_char {
    unsafe {
        let rs_symbol = CStr::from_ptr(symbol).to_str().unwrap();

        let res = match MARKET.as_mut().unwrap().get_book_ticker(rs_symbol) {
            Ok(answer) => format!("{:?}", answer),
            Err(e) => {
                match e.0 {
                    BinanceLibErrorKind::BinanceError(response) => format!("{{ec: \"{}\", errmsg: \"{}\"}}", response.code, response.msg),
                    BinanceLibErrorKind::Msg(msg) => format!("{{ec: 1, errmsg: \"{}\"}}", msg),
                    _ => format!("{{ec: 1, errmsg: \"{}\"}}", e.0),
                }
            },
        };

        CString::new(res).unwrap().into_raw() as *mut c_char
    }
}

#[no_mangle]
// pub fn position_information<S>(&self, symbol: S) -> Result<Vec<PositionRisk>>
pub extern "C" fn get_position_rs(symbol: *const c_char) -> *mut c_char {
    unsafe {
        let rs_symbol_str = CStr::from_ptr(symbol).to_str().unwrap();
        let rs_symbol = match rs_symbol_str {
            "" => None,
            _ => Some(rs_symbol_str.to_string()),
        };
        let res = match ACCOUNT.as_mut().unwrap().position_information(rs_symbol) {
            Ok(answer) => format!("{:?}", answer),
            Err(e) => {
                match e.0 {
                    BinanceLibErrorKind::BinanceError(response) => format!("{{ec: \"{}\", errmsg: \"{}\"}}", response.code, response.msg),
                    BinanceLibErrorKind::Msg(msg) => format!("{{ec: 1, errmsg: \"{}\"}}", msg),
                    _ => format!("{{ec: 1, errmsg: \"{}\"}}", e.0),
                }
            },
        };

        CString::new(res).unwrap().into_raw() as *mut c_char
    }

}
