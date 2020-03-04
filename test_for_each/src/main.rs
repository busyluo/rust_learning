
use std::ops::Drop;

//use tokio::stream::StreamExt;
use futures::stream::{self, StreamExt};

struct Client {
    n: usize
}
impl Client {
    fn new(n: usize) -> Client {
        println!("new:  {}", n);
        Client{ n }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        println!("Drop: {}", self.n);
    }
}

async fn foo() {
    let _a = Client::new(1);
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let stream = stream::repeat(9).take(3);

    let _b = Client::new(2);
    foo().await;

    let each = stream.for_each(|x| {
        println!("for_each: {}", x);
        futures::future::ready(())
    });
    each.await;

    println!("Bye, world!");
}
