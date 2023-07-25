use std::ffi::{CStr, CString};
// use crate::futures::websockets::*;
use std::os::raw::{c_char, c_void};
// use serde_json::Value;
use crate::futures::account::*;
use crate::api::Binance;
use crate::account::OrderSide;
use crate::futures::general::FuturesGeneral;
use crate::futures::websockets::*;
use std::sync::atomic::AtomicBool;

extern fn dummy(_: *const c_char) -> *mut c_char {
    std::ptr::null_mut()
}

static mut FUNC_CPP_FROM_RUST: extern fn(s: *const c_char) -> *mut c_char = dummy;
static mut API_KEY: Option<String> = None;
static mut SECRET_KEY: Option<String> = None;
static mut ACCOUNT: Option<FuturesAccount> = None;
static mut GENERAL: Option<FuturesGeneral> = None;
static mut MARKET: Option<crate::futures::market::FuturesMarket> = None;

///
/// Must be called at beginning
/// 
pub fn init() {
    unsafe {
        API_KEY.get_or_insert_with(|| "yb3hgf0KliueblsrXcwDVgYhfmp7DzUK2m1Jzieg01QHRVCFpg4LBaF1VVYSByTC".to_owned());
        SECRET_KEY.get_or_insert_with(|| "2yNjneC6MRR0Pt9OgzsC6DhygZGRt3yj6n5Pr0SmQB3i1GD2b9ArWynYzspHj5mw".to_owned());
        ACCOUNT.get_or_insert(Binance::new(API_KEY.clone(), SECRET_KEY.clone()));
        GENERAL.get_or_insert(Binance::new(API_KEY.clone(), SECRET_KEY.clone()));
        MARKET.get_or_insert(Binance::new(API_KEY.clone(), SECRET_KEY.clone()));
    }
}

#[no_mangle]
pub extern "C" fn init_from_cpp(callback: extern fn(_: *const c_char) -> *mut c_char) -> i32 {
    unsafe {
        FUNC_CPP_FROM_RUST = callback;
        init();
    }
    0
}

#[no_mangle]
pub extern "C" fn ws_order_book_rs(symbol: *const c_char, data: *mut c_void, callback: extern fn(_: *const c_char, __: *mut c_void) -> *mut c_char) -> i32 {
    let callback_fn = |event: FuturesWebsocketEvent| {
        callback(CString::new(format!("{:?}", event), data).unwrap().into_raw() as *const c_char);
        Ok(())
    };
    let rs_symbol: String;
    unsafe {
        rs_symbol = CStr::from_ptr(symbol).to_str().unwrap().to_owned() + "@depth@0ms";
    }
    let keep_running = AtomicBool::new(true);
    let mut web_socket: FuturesWebSockets<'_> = FuturesWebSockets::new(callback_fn);
    web_socket
        .connect(&FuturesMarket::USDM, &rs_symbol)
        .unwrap();
    web_socket.event_loop(&keep_running).unwrap();
    web_socket.disconnect().unwrap();
    0
}

#[no_mangle]
pub extern "C" fn ws_agg_trade_rs(symbol: *const c_char, data: *mut c_void, callback: extern fn(_: *const c_char, __: *mut c_void) -> *mut c_char) -> i32 {
    let callback_fn = |event: FuturesWebsocketEvent| {
        callback(CString::new(format!("{:?}", event), data).unwrap().into_raw() as *const c_char);
        Ok(())
    };
    let rs_symbol: String;
    unsafe {
        rs_symbol = CStr::from_ptr(symbol).to_str().unwrap().to_owned() + "@aggTrade";
    }
    let keep_running = AtomicBool::new(true);
    let mut web_socket: FuturesWebSockets<'_> = FuturesWebSockets::new(callback_fn);
    web_socket
        .connect(&FuturesMarket::USDM, &rs_symbol)
        .unwrap();
    web_socket.event_loop(&keep_running).unwrap();
    web_socket.disconnect().unwrap();
    0
}

#[no_mangle]
pub extern "C" fn ws_mark_price_rs(symbol: *const c_char, data: *mut c_void, callback: extern fn(_: *const c_char, __: *mut c_void) -> *mut c_char) -> i32 {
    let callback_fn = |event: FuturesWebsocketEvent| {
        callback(CString::new(format!("{:?}", event), data).unwrap().into_raw() as *const c_char);
        Ok(())
    };
    let rs_symbol: String;
    unsafe {
        rs_symbol = CStr::from_ptr(symbol).to_str().unwrap().to_owned() + "@markPrice";
    }
    let keep_running = AtomicBool::new(true);
    let mut web_socket: FuturesWebSockets<'_> = FuturesWebSockets::new(callback_fn);
    web_socket
        .connect(&FuturesMarket::USDM, &rs_symbol)
        .unwrap();
    web_socket.event_loop(&keep_running).unwrap();
    web_socket.disconnect().unwrap();
    0
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
        
        let res = format!("{:?}", ACCOUNT.as_mut().unwrap().cancel_order_with_client_id(rs_symbol, rs_orig_client_order_id));
        CString::new(res).unwrap().into_raw() as *mut c_char
    }
}

#[no_mangle]
// pub fn cancel_order<S>(&self, symbol: S, order_id: u64) -> Result<OrderCanceled>
pub extern "C" fn cancel_order_rs(symbol: *const c_char, order_id: *const c_char) -> *mut c_char {
    unsafe {
        let rs_symbol = CStr::from_ptr(symbol).to_str().unwrap();
        let rs_order_id: &str = CStr::from_ptr(order_id).to_str().unwrap();
        
        let res = format!("{:?}", ACCOUNT.as_mut().unwrap().cancel_order(rs_symbol, rs_order_id.parse::<u64>().unwrap()));
        CString::new(res).unwrap().into_raw() as *mut c_char
    }
}

fn build_custom_order(
                    symbol: *const c_char,
                    order_type: *const c_char, 
                    order_side: *const c_char,
                    qty: *const c_char,
                    price: *const c_char,
                    stop_price: *const c_char,
                    time_in_force: *const c_char,
                    activation_price: *const c_char,
                    callback_rate: *const c_char,
                    close_position: *const c_char) -> CustomOrderRequest {
    unsafe {
        let rs_symbol = CStr::from_ptr(symbol).to_str().unwrap();

        let rs_qty_str = CStr::from_ptr(qty).to_str().unwrap();
        let rs_qty = match rs_qty_str {
            "" => None,
            _ => Some(rs_qty_str.parse::<f64>().unwrap()),
        };

        let rs_price_str = CStr::from_ptr(price).to_str().unwrap();
        let rs_price = match rs_price_str {
            "" => None,
            _ => Some(rs_price_str.parse::<f64>().unwrap()),
        };

        let rs_stop_price_str = CStr::from_ptr(stop_price).to_str().unwrap();
        let rs_stop_price = match rs_stop_price_str {
            "" => None,
            _ => Some(rs_stop_price_str.parse::<f64>().unwrap()),
        };

        let rs_callback_rate_str = CStr::from_ptr(callback_rate).to_str().unwrap();
        let rs_callback_rate = match rs_callback_rate_str {
            "" => None,
            _ => Some(rs_callback_rate_str.parse::<f64>().unwrap()),
        };


        let rs_activation_price_str = CStr::from_ptr(activation_price).to_str().unwrap();
        let rs_activation_price = match rs_activation_price_str {
            "" => None,
            _ => Some(rs_activation_price_str.parse::<f64>().unwrap()),
        };

        let rs_order_side_str = CStr::from_ptr(order_side).to_str().unwrap();
        let rs_order_side = match rs_order_side_str {
            "buy" => OrderSide::Buy,
            "sell" => OrderSide::Sell,
            &_ => panic!("unknown order side"),
        };

        let rs_order_type_str = CStr::from_ptr(order_type).to_str().unwrap();
        let rs_order_type = match rs_order_type_str {
            "market" => OrderType::Market,
            "limit" => OrderType::Limit,
            "stop" => OrderType::Stop,
            "stop_market" => OrderType::StopMarket,
            "take_profit" => OrderType::TakeProfit,
            "take_profit_market" => OrderType::TakeProfitMarket,
            "trailing_stop_market" => OrderType::TrailingStopMarket,
            &_ => panic!("unknown order type"),
        };

        let rs_time_in_force_str: &str = CStr::from_ptr(time_in_force).to_str().unwrap();
        let rs_time_in_force = match rs_time_in_force_str {
            "gtc" => Some(TimeInForce::GTC),
            "ioc" => Some(TimeInForce::IOC),
            "fok" => Some(TimeInForce::FOK),
            "gtx" => Some(TimeInForce::GTX),
            &_ => None,
        };

        let rs_close_position_str: &str = CStr::from_ptr(close_position).to_str().unwrap();
        let rs_close_position = match rs_close_position_str {
            "" => None,
            _ => Some(rs_close_position_str.parse::<bool>().unwrap()),
        };

        if rs_order_type != "stop_market" && rs_order_type != "take_profit_market" {
            if rs_order_type != "stop" && rs_order_type != "take_profit" {
                stop_price = None;
            }
            rs_close_position = None;
        }

        if rs_order_type != "trailing_stop_market" {
            rs_callback_rate = None;
            rs_activation_price = None;
        }

        if rs_close_position == Some(true) {
            rs_qty = None;
        }
        
        CustomOrderRequest {
            symbol: rs_symbol.to_owned(),
            side: rs_order_side,
            position_side: Some(PositionSide::Both),
            order_type: rs_order_type,
            time_in_force: rs_time_in_force,
            qty: rs_qty,
            reduce_only: None,
            price: rs_price,
            stop_price: rs_stop_price,
            close_position: rs_close_position,
            activation_price: rs_activation_price,
            callback_rate: rs_callback_rate,
            working_type: None,
            price_protect: None
        }
    }
}

#[no_mangle]
// pub fn custom_order<S, F>(&self, symbol: S, qty: F, price: f64, stop_price: Option<f64>, order_side: OrderSide,
//    order_type: OrderType, time_in_force: TimeInForce, new_client_order_id: Option<String>, ) -> Result<Transaction>
pub extern "C" fn custom_order_rs(
                    symbol: *const c_char,
                    order_type: *const c_char, 
                    order_side: *const c_char,
                    qty: *const c_char,
                    price: *const c_char,
                    stop_price: *const c_char,
                    time_in_force: *const c_char,
                    activation_price: *const c_char,
                    callback_rate: *const c_char,
                    close_position: *const c_char) -> *mut c_char {
    let res: String;
    unsafe {
        res = format!("{:?}", ACCOUNT.as_mut().unwrap().custom_order(build_custom_order(symbol, order_type, order_side, qty, price, stop_price, time_in_force, activation_price, callback_rate, close_position)));
    }
    CString::new(res).unwrap().into_raw() as *mut c_char
}


#[no_mangle]
// pub fn exchange_info(&self) -> Result<ExchangeInformation>
pub extern "C" fn exchange_info_rs() -> *mut c_char {
    unsafe {
        let res = format!("{:?}", GENERAL.as_mut().unwrap().exchange_info());
        CString::new(res).unwrap().into_raw() as *mut c_char
    }
}

#[no_mangle]
// pub fn exchange_info(&self) -> Result<ExchangeInformation>
pub extern "C" fn account_balance_rs() -> *mut c_char {
    unsafe {
        let res = format!("{:?}", ACCOUNT.as_mut().unwrap().account_balance());
        CString::new(res).unwrap().into_raw() as *mut c_char
    }
}

#[no_mangle]
// pub fn exchange_info(&self) -> Result<ExchangeInformation>
pub extern "C" fn cancel_all_open_orders_rs(symbol: *const c_char) -> *mut c_char {
    unsafe {
        let rs_symbol = CStr::from_ptr(symbol).to_str().unwrap();
        let res = format!("{:?}", ACCOUNT.as_mut().unwrap().cancel_all_open_orders(rs_symbol));
        CString::new(res).unwrap().into_raw() as *mut c_char
    }
}

#[no_mangle]
// pub fn get_custom_depth<S>(&self, symbol: S, depth: u64) -> Result<OrderBook>
pub extern "C" fn get_custom_depth_rs(symbol: *const c_char, depth: *const c_char) -> *mut c_char {    
    unsafe {
        let rs_symbol = CStr::from_ptr(symbol).to_str().unwrap();
        let rs_depth = CStr::from_ptr(depth).to_str().unwrap();
        let res = format!("{:?}", MARKET.as_mut().unwrap().get_custom_depth(rs_symbol, rs_depth.parse::<u64>().unwrap()));
        CString::new(res).unwrap().into_raw() as *mut c_char
    }
}

#[no_mangle]
// pub fn get_price<S>(&self, symbol: S) -> Result<SymbolPrice>
pub extern "C" fn get_price_rs(symbol: *const c_char) -> *mut c_char {
    unsafe {
        let rs_symbol = CStr::from_ptr(symbol).to_str().unwrap();

        let res = format!("{:?}", MARKET.as_mut().unwrap().get_price(rs_symbol));
        CString::new(res).unwrap().into_raw() as *mut c_char
    }
}

#[no_mangle]
// pub fn get_book_ticker<S>(&self, symbol: S) -> Result<Tickers>
pub extern "C" fn get_book_ticker_rs(symbol: *const c_char) -> *mut c_char {
    unsafe {
        let rs_symbol = CStr::from_ptr(symbol).to_str().unwrap();

        let res = format!("{:?}", MARKET.as_mut().unwrap().get_book_ticker(rs_symbol));
        CString::new(res).unwrap().into_raw() as *mut c_char
    }
}
