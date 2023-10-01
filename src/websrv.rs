use crate::transport::Transport;
use log::trace;
use std::net::SocketAddr;

struct Packet {
    packet_type: u8,
    //data: [u8],
}

pub struct Websrv {
    tp: Transport,
}

impl Websrv {
    pub fn new() -> Websrv {
        trace!("Initializing");
        let tp = Transport::new(1111, on_packet);
        Websrv { tp }
    }

    fn on_packet(self, pkt: Packet, src_addr: SocketAddr) {
        trace!("Processing packet");
    }

    pub fn stop() {
        //tp.stop();
    }
}
