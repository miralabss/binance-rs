#include <stdio.h>
#include <dlfcn.h>


// Globals
typedef char* (*F_C_STR)(const char*);
typedef char* (*F_0_C_STR)();
typedef char* (*F_1_C_STR)(const char*);
typedef char* (*F_2_C_STR)(const char*, const char*);
typedef char* (*F_10_C_STR)(const char*, const char*, const char*, const char*, const char*, const char*, const char*, const char*, const char*, const char*);
typedef int (*F_RUST_WITH_CB)(F_C_STR);
typedef int (*F_1_RUST_WITH_CB)(const char*, F_C_STR);

F_RUST_WITH_CB init_from_cpp = NULL;
F_C_STR rust_from_cpp = NULL;

F_1_RUST_WITH_CB ws_order_book_rs = NULL;
F_1_RUST_WITH_CB ws_mark_price_rs = NULL;
F_1_RUST_WITH_CB ws_agg_trade_rs = NULL;
F_2_C_STR cancel_order_with_client_id_rs = NULL;
F_2_C_STR cancel_order_rs = NULL;
F_10_C_STR new_order_rs = NULL;
F_0_C_STR exchange_info_rs = NULL;
F_0_C_STR account_balance_rs = NULL;
F_2_C_STR get_custom_depth_rs = NULL;
F_1_C_STR get_price_rs = NULL;
F_1_C_STR get_book_ticker_rs = NULL;
F_1_C_STR cancel_all_open_orders_rs = NULL;

void* gHandler_LibBinance = NULL;

char* cppFromRust(const char* s) {
    printf("Received String From Rust : %s \n", s);
    return NULL;
}

template <typename T>
int handler_dlsym(T* f, void *__handle, const char *__symbol) {
    void* ptr = dlsym(__handle, __symbol);
    if (ptr) {
        *f = (T) ptr;
        return 0;
    } else {
        printf("Error: symbol not found (%s)\n", __symbol);
        return -1;
    }
}

int load_library() {
    gHandler_LibBinance = dlopen("./libbinance.so", RTLD_NOW);
    if (!gHandler_LibBinance) {
        printf("Error: failed to load library\n");
        return -1;
    }
    if (handler_dlsym(&init_from_cpp, gHandler_LibBinance, "init_from_cpp")) {
        return -1;
    }
    if (handler_dlsym(&rust_from_cpp, gHandler_LibBinance, "rust_from_cpp")) {
        return -1;
    }
    if (handler_dlsym(&ws_order_book_rs, gHandler_LibBinance, "ws_order_book_rs")) {
        return -1;
    }
    if (handler_dlsym(&ws_mark_price_rs, gHandler_LibBinance, "ws_mark_price_rs")) {
        return -1;
    }
    if (handler_dlsym(&ws_agg_trade_rs, gHandler_LibBinance, "ws_agg_trade_rs")) {
        return -1;
    }
    if (handler_dlsym(&cancel_order_with_client_id_rs, gHandler_LibBinance, "cancel_order_with_client_id_rs")) {
        return -1;
    }
    if (handler_dlsym(&cancel_order_rs, gHandler_LibBinance, "cancel_order_rs")) {
        return -1;
    }
    if (handler_dlsym(&new_order_rs, gHandler_LibBinance, "custom_order_rs")) {
        return -1;
    }
    if (handler_dlsym(&exchange_info_rs, gHandler_LibBinance, "exchange_info_rs")) {
        return -1;
    }
    if (handler_dlsym(&account_balance_rs, gHandler_LibBinance, "account_balance_rs")) {
        return -1;
    }
    if (handler_dlsym(&get_custom_depth_rs, gHandler_LibBinance, "get_custom_depth_rs")) {
        return -1;
    }
    if (handler_dlsym(&cancel_all_open_orders_rs, gHandler_LibBinance, "cancel_all_open_orders_rs")) {
        return -1;
    }
    if (handler_dlsym(&get_price_rs, gHandler_LibBinance, "get_price_rs")) {
        return -1;
    }
    if (handler_dlsym(&get_book_ticker_rs, gHandler_LibBinance, "get_book_ticker_rs")) {
        return -1;
    }
    return 0;
}

int close_library() {
    dlclose(gHandler_LibBinance);
    return 0;
}

char* cb(const char* input) {
    printf("bebra\n");
    printf("%s\n", input);
    return NULL;
}

int main() {
    if (load_library()) {
        printf("Fail to init rust library\n");
        return -1;
    }

    init_from_cpp(rust_from_cpp);

    ws_order_book_rs("btcusdt", cb);
//    printf("%s\n", get_custom_depth_rs("BTCUSDT", "5"));

    close_library();
}

