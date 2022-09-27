use ggrs::SessionState;
use ggrs::{P2PSession, PlayerType, SessionBuilder, UdpNonBlockingSocket};

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::SocketAddr;
use async_executor::LocalExecutor;
use futures::*;
use matchbox_socket::WebRtcSocket;
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
    pub session: Option<P2PSession<GGRSConfig>>,
    pub local_handle: usize,
    pub local_executor: LocalExecutor<'a>,
    pub socket: Option<WebRtcSocket>
}


impl<'a> Net<'a> {

    pub fn launch_session() -> Net<'a> {
        let room_url = "ws://127.0.0.1:3536/next_2";
        let (socket, message_loop) = WebRtcSocket::new(room_url);
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

    pub fn is_running(&self) -> bool {
        return self.session.as_ref().unwrap().current_state() == SessionState::Running;
    }

    pub fn add_local_input(&mut self, handle: usize, inputs: NetInput) {
        self.session.as_mut().unwrap().add_local_input(handle, inputs).unwrap();
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
        let socket = self.socket.take().unwrap();
        
        let players = socket.players();
        // consume the socket (currently required because ggrs takes ownership of its socket)

        let max_prediction = 12;

        // create a GGRS P2P session
        let mut sess_build = SessionBuilder::<GGRSConfig>::new()
            .with_num_players(2)
            .with_max_prediction_window(max_prediction)
            .with_input_delay(2)
            .with_fps(60)
            .expect("invalid fps");

        for (i, player) in players.into_iter().enumerate() {
            match player {
                PlayerType::Local => {
                    sess_build = sess_build.add_player(PlayerType::Local, i).unwrap();
                    self.local_handle = i;
//                    local_handle = i;
                },
                PlayerType::Remote(addr) => {
                    println!("{:?}", addr);
                    sess_build = sess_build.add_player(PlayerType::Remote(addr), i).unwrap();
                },
                _ => {
                    
                }
            }
        }

    // start the GGRS session
        let sess = sess_build
        .start_p2p_session(socket)
        .expect("failed to start session");
        
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

