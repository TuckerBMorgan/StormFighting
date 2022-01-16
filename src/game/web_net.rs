use ggrs::{P2PSession, PlayerType, NonBlockingSocket};

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::SocketAddr;
use matchbox_socket::WebRtcNonBlockingSocket;
use futures::executor::block_on;
use async_executor::*;
pub const FPS: f64 = 60.0;
pub const INPUT_SIZE: usize = std::mem::size_of::<[u8;2]>();
use std::sync::Arc;
use futures::lock::Mutex;

type MessageLoop = dyn futures::Future<Output = ()>;

pub struct Net {
    pub session: P2PSession,
    pub message_loop: Box<MessageLoop>,
    pub local_handle: usize
}

impl Net {
    pub fn new(session: P2PSession, message_loop:Box<MessageLoop>, local_handle: usize) -> Net {
        Net {
            session,
            message_loop,
            local_handle
        }
    }
}

pub fn launch_session()-> Net {
    let (mut socket, message_loop) = WebRtcNonBlockingSocket::new("ws://127.0.0.1:3536");
    let mut sess = P2PSession::new_with_socket(1, INPUT_SIZE, 16, socket);
    let message_loop = Arc::new(Mutex::new(Box::pin(message_loop)));
    tokio::spawn(message_loop.clone());
    return Net::new(sess, Box::new(message_loop), 1);
}   
 