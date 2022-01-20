use ggrs::{P2PSession, PlayerType, NonBlockingSocket};

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::SocketAddr;

pub const FPS: f64 = 60.0;
pub const INPUT_SIZE: usize = std::mem::size_of::<[u8;2]>();
pub struct Net<'a> {
    pub session: P2PSession,
    pub local_handle: usize
}

impl<'a> Net<'a> {
    pub fn new(session: P2PSession, local_handle: usize) -> Net<'a> {
        Net {
            session,
            local_handle
        }
    }

    pub fn launch_session() -> Net<'a> {
        //Connect to the Cupid server
       // let mut stream = TcpStream::connect("24.19.122.147:7878").unwrap();
        let mut stream = TcpStream::connect("192.168.0.20:7878").unwrap();
        //let listener = TcpListener::bind("192.168.0.20:7878").unwrap();
        let mut players = vec![String::from("localhost"), String::from("localhost")];
    
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
    
        // create a GGRS session
        let mut sess = P2PSession::new(num_players as u32, INPUT_SIZE,8, stream.local_addr().unwrap().port()).unwrap();
    
        // turn on sparse saving
        sess.set_sparse_saving(false).unwrap();
    
        // set FPS (default is 60, so this doesn't change anything as is)
        sess.set_fps(FPS as u32).unwrap();
    
        // add players
        for (i, player_addr) in players.iter().enumerate() {
            // local player
            if player_addr == "localhost" {
                sess.add_player(PlayerType::Local, i).unwrap();
                local_handle = i;
            } else {
                // remote players
                let remote_addr: SocketAddr = player_addr.parse().unwrap();
                sess.add_player(PlayerType::Remote(remote_addr), i).unwrap();
            }
        }
    
        // set input delay for the local player
        sess.set_frame_delay(2, local_handle).unwrap();
    
        // set change default expected update frequency
        sess.set_fps(FPS as u32).unwrap();
        // start the GGRS session
        sess.start_session().unwrap();
        return Net::new(sess, local_handle);
    }
    pub fn tick(&mut self) {
        self.session.poll_remote_clients();
    }
}

