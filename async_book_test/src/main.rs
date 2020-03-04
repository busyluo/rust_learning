use std::future::Future;

mod ref_cell;

use std::sync::mpsc;

fn main() {
    println!("Hello, world!");


    let (a, b) = mpsc::channel();
}

async fn foo(x: &u8) -> u8 { *x }

//fn bad() -> impl Future<Output = u8> {
//    let x = 5;
//    foo(&x) // ERROR: `x` does not live long enough
//}

fn good() -> impl Future<Output=u8> {
    let x = 5;
    async move {
        foo(&x).await
    }
}

async fn yes() -> u8 {
    let x = 5;
    foo(&x).await
}

//C:\Users\24880\.rustup\toolchains\stable-x86_64-pc-windows-gnu\lib\rustlib\src\rust\src\libstd\lib.rs
//C:\Users\24880\.rustup\toolchains\stable-x86_64-pc-windows-gnu\lib\rustlib\src\rust\src\libcore\lib.rs