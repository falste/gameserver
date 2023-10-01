use log::{error, info, trace};
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use bytes::{BufMut, Bytes, BytesMut};

type RecvCallback = fn(Bytes, SocketAddr);

const BUFSIZE: usize = 100;

fn recv_func(port: u16, recv_callback: RecvCallback) {
    let bind_address = SocketAddr::from(([0, 0, 0, 0], port));
    let socket = UdpSocket::bind(bind_address).unwrap();
    info!("Listening on port {} (UDP)", port);

    let mut buffer: [u8; BUFSIZE] = [0; BUFSIZE];
    loop {
        match socket.recv_from(&mut buffer) {
            Ok((size, src_addr)) => {
                let mut msg = BytesMut::new();
                msg.put_slice(&buffer[..size]);
                trace!("Received message of length {} from {}", size, src_addr);

                recv_callback(msg.freeze(), src_addr);
            }
            Err(e) => {
                error!("Error while receiving UDP message: {:?}", e);
            }
        }
    }
}

pub struct Transport {
    mutex: Arc<Mutex<bool>>,
    thread_handle: JoinHandle<()>,
}

impl Transport {
    pub fn new(port: u16, recv_callback: RecvCallback) -> Transport {
        trace!("New");
        let mutex = Arc::new(Mutex::new(true));

        let thread_handle = thread::Builder::new()
            .name("Transport receiver thread".to_string())
            .spawn(move || {
                recv_func(port, recv_callback);
            })
            .unwrap();

        Transport {
            mutex,
            thread_handle,
        }
    }

    pub fn send(&self, msg: Bytes, target: SocketAddr) {
        let bind_address = SocketAddr::from(([0, 0, 0, 0], 0));
        let socket = UdpSocket::bind(bind_address).unwrap();

        let data: &[u8] = &msg;
        socket.send_to(data, target);
    }

    pub fn stop(self) {
        trace!("Stopping");
        let mut running = self.mutex.lock().unwrap();
        *running = false;
        self.thread_handle.join().unwrap();
    }
}
