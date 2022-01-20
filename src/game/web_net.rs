use ggrs::{P2PSession, PlayerType, NonBlockingSocket};

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::SocketAddr;
use async_executor::LocalExecutor;
use future::blocks_on;
use matchbox_socket::*;

pub const FPS: f64 = 60.0;
pub const INPUT_SIZE: usize = std::mem::size_of::<[u8;2]>();
pub struct Net<'a> {
    pub session: P2PSession,
    pub local_handle: usize,
    pub local_executor: LocalExecutor<'a>
}


impl<'a> Net<'a> {
    pub fn new(session: P2PSession, local_handle: usize, local_executor: LocalExecutor<'a>) -> Net<'a> {
        Net {
            session,
            local_handle,
            local_executor
        }
    }

    pub fn launch_session() -> Net<'a> {
        let mut local_handle = 0;
        let (mut socket, message_loop) = WebRtcNonBlockingSocket::new("ws://127.0.0.1:3536/ggssee");
        let local_ex = LocalExecutor::new();
        block_on(message_loop);
        //let task = local_ex.spawn(message_loop);
        //task.detach();

        loop {
            local_ex.try_tick();
            socket.accept_new_connections();
            let connected_peers = socket.connected_peers().len();
            let remaining = 2 - (connected_peers + 1);
            if remaining == 0 {
                break;
            }
        }
        println!("---");
        let players = socket.players();
        //    let (mut socket, _) = connect(Url::parse("ws://192.168.0.20:9001").unwrap())?;
        let mut sess = P2PSession::new_with_socket(2, INPUT_SIZE, 16, socket);
        // turn on sparse saving
        sess.set_sparse_saving(false).unwrap();
    
        // set FPS (default is 60, so this doesn't change anything as is)
        sess.set_fps(FPS as u32).unwrap();

        // add players
        for (i, player) in players.into_iter().enumerate() {
            sess
                .add_player(player, i)
                .expect("failed to add player");
    
            if player == PlayerType::Local {
                // set input delay for the local player
                sess.set_frame_delay(2, i).unwrap();
            }
        }
    
        // set input delay for the local player
        sess.set_frame_delay(2, local_handle).unwrap();
    
        // set change default expected update frequency
        sess.set_fps(FPS as u32).unwrap();
        // start the GGRS session
        sess.start_session().unwrap();

        return Net::new(sess, 1, local_ex);
    }
    
    pub fn tick(&mut self) {
        self.local_executor.try_tick();
        self.session.poll_remote_clients();
    }
}

