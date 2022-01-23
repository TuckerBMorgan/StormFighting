use ggrs::{P2PSession, PlayerType, NonBlockingSocket};

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::SocketAddr;
use async_executor::LocalExecutor;
use futures::*;
use matchbox_socket::*;
use crate::game::web_net::executor::block_on;
use super::*;
pub const FPS: f64 = 60.0;
pub const INPUT_SIZE: usize = std::mem::size_of::<[u8;2]>();

pub enum NetState {
    Connecting,
    Live
}


pub struct Net<'a> {
    pub state: NetState,
    pub session: Option<P2PSession<Round>>,
    pub local_handle: usize,
    pub local_executor: LocalExecutor<'a>,
    pub socket: Option<WebRtcNonBlockingSocket>
}


impl<'a> Net<'a> {

    pub fn launch_session() -> Net<'a> {
        let (mut socket, message_loop) = WebRtcNonBlockingSocket::new("wss://test-match.helsing.studio");
        let local_executor = LocalExecutor::new();
        let task = local_executor.spawn(message_loop);
        task.detach();
        Net {
            local_handle: 0,
            state: NetState::Connecting,
            session: None,
            local_executor,
            socket: Some(socket)
        }
    }

    
    pub fn connecting_tick(&mut self) {
        let mut local_handle = 0;
        self.local_executor.try_tick();
        self.socket.as_mut().unwrap().accept_new_connections();
        let connected_peers = self.socket.as_mut().unwrap().connected_peers().len();
        let remaining = 2 - (connected_peers + 1);
        if remaining != 0 {
            return;
        }
        
        let players = self.socket.as_mut().unwrap().players();
        //    let (mut socket, _) = connect(Url::parse("ws://192.168.0.20:9001").unwrap())?;
        let mut sess = P2PSession::<Round>::new_with_socket(2, INPUT_SIZE, 16, self.socket.take().unwrap());
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
                self.local_handle = i;
            }
        }
    
        // set input delay for the local player
   //     sess.set_frame_delay(2, local_handle).unwrap();
    
        // set change default expected update frequency
        sess.set_fps(FPS as u32).unwrap();
        // start the GGRS session
        sess.start_session().unwrap();

        self.session = Some(sess);
        self.state = NetState::Live;
    }
    
    pub fn live_tick(&mut self) {
        self.local_executor.try_tick();
        self.session.as_mut().unwrap().poll_remote_clients();
    }

    pub fn tick(&mut self) {
        match self.state {
            NetState::Connecting => {
                self.connecting_tick();
            },
            NetState::Live => {
                self.live_tick();
            }
        }
    }
}

