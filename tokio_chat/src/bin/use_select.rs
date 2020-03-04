#![allow(dead_code)]

use tokio::net::{TcpListener, TcpStream};
use std::thread;
use std::error::Error;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_util::codec::{Framed, LinesCodec, LinesCodecError};
use futures::{SinkExt, StreamExt};
use tokio::sync::Mutex;
use futures::future::{FutureExt, Either};
use futures::future;

//use tokio::net::{TcpListener, TcpStream};
//use tokio::stream::{Stream, StreamExt};
//use tokio::sync::{mpsc, Mutex};
//use tokio_util::codec::{Framed, LinesCodec, LinesCodecError};
//
////use futures::SinkExt;
//use std::collections::HashMap;
//use std::env;
//use std::error::Error;
//use std::io;
//use std::net::SocketAddr;
//use std::pin::Pin;
//use std::sync::{Arc};
//use std::task::{Context, Poll};

//type Tx = mpsc::Sender<String>;
type Tx = mpsc::UnboundedSender<String>;

struct Shared {
    peers: HashMap<SocketAddr, Tx>,
}

impl Shared {
    fn new() -> Shared {
        Shared {
            peers: HashMap::new()
        }
    }

    fn broadcast(&self, from: SocketAddr, msg: String) {
        for peer in &self.peers {
            if *peer.0 != from {
                peer.1.send(msg.clone());
            }
        }
    }
}

enum Message {
    Received(String),
    Broadcast(String),
    None,
}

async fn process(
    state: Arc<Mutex<Shared>>,
    stream: TcpStream,
    addr: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    let mut lines = Framed::new(stream, LinesCodec::new());

    lines.send("Please enter you username:".to_string()).await?;

    let username = match lines.next().await {
        Some(Ok(line)) => line,
        _ => {
            return Err(format!("Failed to get username from {}. Client disconnected.", addr).into());
        }
    };
    println!("user '{}' login.", username);

    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    //state.lock().await.peers.insert(addr, tx);
    {
        let mut state = state.lock().await;
        state.peers.insert(addr, tx);
        state.broadcast(addr, format!("{} joined in.", username));
    }

    loop {
        let recv = Box::pin(rx.recv());
        let line = Box::pin(lines.next());
        let cb = future::select(recv, line).map(|either| {
            match either {
                Either::Left((Some(msg), _)) =>  {
                    Message::Received(msg)
                }
                Either::Right((Some(Ok(msg)), _)) => {
                    Message::Broadcast(msg)
                }
                _ => Message::None
            }
        });
        match cb.await {
            Message::Received(msg) => {
                println!("send to {}", username);
                lines.send(msg).await?;
            }
            Message::Broadcast(msg) => {
                println!("msg: {}", msg);
                state.lock().await.broadcast(addr, format!("{}: {}", username, msg));
            }
            _ => {}
        };
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:6688";
    let mut listener = TcpListener::bind(&addr).await?;

    println!("server running on {}", addr);

    let shared_state = Arc::new(Mutex::new(Shared::new()));

    loop {
        let (stream, addr) = listener.accept().await?;
        let state = shared_state.clone();
        tokio::spawn(async move {
            if let Err(e) = process(state, stream, addr).await {
                println!("An error occurred; error = {:?}", e);
            }
        });
    }
}
