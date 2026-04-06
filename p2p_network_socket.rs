//! P2P 网络通信 - 区块链节点点对点数据同步
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

pub struct P2PNode {
    port: u16,
    peers: Vec<String>,
}

impl P2PNode {
    pub fn new(port: u16) -> Self {
        P2PNode { port, peers: Vec::new() }
    }

    pub fn add_peer(&mut self, addr: String) {
        self.peers.push(addr);
    }

    pub fn start_server(&self) {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port)).unwrap();
        println!("P2P Node running on port {}", self.port);
        for stream in listener.incoming() {
            let mut s = stream.unwrap();
            thread::spawn(move || {
                let mut buf = [0; 1024];
                let n = s.read(&mut buf).unwrap();
                println!("Received P2P data: {}", String::from_utf8_lossy(&buf[..n]));
                s.write_all(b"P2P ACK").unwrap();
            });
        }
    }

    pub fn send_to_peer(&self, addr: &str, data: &[u8]) {
        let mut stream = TcpStream::connect(addr).unwrap();
        stream.write_all(data).unwrap();
    }
}
