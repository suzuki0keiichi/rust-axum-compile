use std::net::SocketAddr;
use std::sync::{Arc, RwLock};

use axum::{handler::get, response::IntoResponse, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hoge", get(get_hoge));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

struct HogeStruct {}

impl HogeStruct {
    fn hoge_method(&self) -> String {
        "hello".to_string()
    }

    async fn async_hoge_method(&self) -> String {
        "hello".to_string()
    }
}

async fn async_hoge_function() -> String {
    "hello".to_string()
}

async fn get_hoge() -> impl IntoResponse {
    // 1. コンパイル通る(asyncだけどただのfunction)
    async_hoge_function().await

    // 2. コンパイル通る(asyncじゃない)
    // let hoge_struct = Arc::new(RwLock::new(HogeStruct{}));
    // let message = hoge_struct.read().unwrap().hoge_method();
    // message

    // 3. コンパイル通る(asyncだけどRwLockを使っていない)
    // let hoge_struct = HogeStruct {};
    // hoge_struct.async_hoge_method().await

    // 4. コンパイル通らない(RwLockから取得且つasync)
    // let hoge_struct = Arc::new(RwLock::new(HogeStruct {}));
    // let message = hoge_struct
    //     .read()
    //     .unwrap()
    //     .async_hoge_method()
    //     .await
    //     .clone();
    // message
}
