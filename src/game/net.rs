use ggrs::SessionState;
use ggrs::{P2PSession, PlayerType, SessionBuilder, UdpNonBlockingSocket};

use std::io::prelude::*;
use std::marker::PhantomData;
use std::net::TcpStream;
use std::net::SocketAddr;
use crate::*;

pub const FPS: f64 = 60.0;


pub enum NetState {
    Connecting,
    Live
}


pub struct Net<'a> {
    pub session: Option<P2PSession<GGRSConfig>>,
    pub local_handle: usize,
    marker: PhantomData<&'a ()>,
    pub state: NetState
}

impl<'a> Net<'a> {
    pub fn new(session: P2PSession<GGRSConfig>, local_handle: usize) -> Net<'a>  {
        Net {
            session: Some(session),
            local_handle,
            marker: PhantomData,
            state: NetState::Live
        }
    }

    pub fn is_running(&self) -> bool {
        return self.session.as_ref().unwrap().current_state() == SessionState::Running;
    }

    pub fn add_local_input(&mut self, handle: usize, inputs: NetInput) {
        self.session.as_mut().unwrap().add_local_input(handle, inputs).unwrap();
    }

    pub fn launch_session() -> Net<'a> {
        //Connect to the Cupid server
       // let mut stream = TcpStream::connect("24.19.122.147:7878").unwrap();
        let mut stream = TcpStream::connect("192.168.0.20:7878").unwrap();

        let mut players = vec![String::from("localhost")];
    
        let mut buffer = [0;512];
        let mut message = vec![];
        stream.read(&mut buffer).unwrap();
        let mut has_seen_one = false;
        for value in buffer {
            if value == 96 {
                if has_seen_one == false {
                    has_seen_one = true;
                }
                else {
                    let test : String = message.iter().collect();
                    let parts : Vec<String> = test.split('#').map(|x|x.to_string()).collect();
    
                    let player_pos:char = parts[1].chars().nth(0).unwrap();
                    if player_pos == '\u{1}' {    
                        players.insert(0, parts[0].clone())
                    }
                    else {
                        players.push(parts[0].clone());
                    }
                    break;
                }
            }
            else {
                message.push(value as char);
            }
        }

        //Now that we have the info kill our connection
        let _ = stream.shutdown(std::net::Shutdown::Both);
    
        // read cmd line arguments
        let mut local_handle = 0;
        let num_players = 2;//This is a peer to peer fighting game, there will only ever be 2 players
        assert!(num_players > 0);
        let mut sess = SessionBuilder::<GGRSConfig>::new()
            .with_num_players(num_players)
            .with_fps(FPS as usize).unwrap()
            .with_input_delay(2);

        // add players
        for (i, player_addr) in players.iter().enumerate() {
            // local player
            if player_addr == "localhost" {
                sess = sess.add_player(PlayerType::Local, i).unwrap();
                local_handle = i;
            } else {

                // remote players
                sess = sess.add_player(PlayerType::Remote(player_addr), i).unwrap();
            }
        }

        let socket = UdpNonBlockingSocket::bind_to_port(stream.local_addr().unwrap().port()).unwrap();
        let sess = sess.start_p2p_session(socket).unwrap();
        return Net::new(sess, local_handle);
    }

    pub fn tick(&mut self) {
        self.session.as_mut().unwrap().poll_remote_clients();
    }
}

