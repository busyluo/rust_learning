#![allow(dead_code)]

//use tokio::net::{TcpListener, TcpStream};
//use std::{thread, io};
//use std::error::Error;
//use std::collections::HashMap;
//use std::net::SocketAddr;
//use std::sync::{Arc};
//use tokio::sync::mpsc;
//use tokio_util::codec::{Framed, LinesCodec, LinesCodecError};
//// use tokio::stream::{StreamExt, Stream};
//use tokio::sync::Mutex;
//use futures::future::{FutureExt, Either};
//use futures::{future, Stream, StreamExt};
//use std::pin::Pin;
//use futures::SinkExt;
//use futures::task::{Context, Poll};

use tokio::net::{TcpListener, TcpStream};
use tokio::stream::{Stream, StreamExt};
use tokio::sync::{mpsc, Mutex};
use tokio_util::codec::{Framed, LinesCodec, LinesCodecError};

use futures::{SinkExt};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::{Arc};
use std::task::{Context, Poll};

type Tx = mpsc::UnboundedSender<String>;
type Rx = mpsc::UnboundedReceiver<String>;

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

struct Peer {
    rx: Rx,
    lines: Framed<TcpStream, LinesCodec>
}

impl Peer {
    async fn new(state: Arc<Mutex<Shared>>, lines: Framed<TcpStream, LinesCodec>) -> io::Result<Peer> {

        let addr = lines.get_ref().peer_addr()?;
        let (tx, rx) = mpsc::unbounded_channel();

        state.lock().await.peers.insert(addr, tx);

        Ok(Peer {
            rx,
            lines,
        })
    }
}


impl Stream for Peer {
    type Item = Message;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Poll::Ready(Some(v)) = self.rx.poll_recv(cx) {
            return Poll::Ready(Some(Message::Received(v)));
        }

        let result = futures::ready!(Pin::new(&mut self.lines).poll_next(cx));
        match result {
            Some(Ok(msg)) => Poll::Ready(Some(Message::Broadcast(msg))),
            _ => Poll::Ready(None)
        }
    }
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

    // let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    let mut peer = Peer::new(state.clone(), lines).await?;

    state.lock().await.broadcast(addr, format!("{} joined in.", username));

    while let Some(res) = peer.next().await {
        match res {
            Message::Received(msg) => {
                println!("send to {}", username);
                peer.lines.send(msg).await?;
            }
            Message::Broadcast(msg) => {
                println!("msg: {}", msg);
                state.lock().await.broadcast(addr, format!("{}: {}", username, msg));
            }
            _ => {}
        };
    }
    Ok(())
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
