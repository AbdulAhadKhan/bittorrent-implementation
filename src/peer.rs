use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

// The byte size of a Peer Protocol is calculable
// using the information in the specification:
// https://www.bittorrent.org/beps/bep_0003.html#peer-protocol.
// This totals to 68 bytes.
const MESSAGE_SIZE: usize = 68;

type Message = [u8];

#[repr(C)]
#[derive(Debug)]
pub struct PeerProtocol {
    pub length: u8,
    pub protocol_string: [u8; 19],
    pub reserved: [u8; 8],
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20],
}

impl PeerProtocol {
    pub fn new(info_hash: [u8; 20], peer_id: &str) -> Self {
        let peer_id = hex::decode(peer_id).unwrap();
        let peer_id = peer_id.try_into().unwrap();

        PeerProtocol {
            length: 19,
            protocol_string: *b"BitTorrent protocol",
            reserved: [0; 8],
            info_hash,
            peer_id,
        }
    }

    pub async fn connect(&mut self, address: &str) {
        let mut message = self.as_bytes();
        let mut stream = TcpStream::connect(address).await.unwrap();

        stream.write_all(&mut message).await.unwrap();
        stream.readable().await.unwrap();

        message.fill(0);
        let _ = stream.read_exact(&mut message).await.unwrap();
    }

    fn as_bytes(&mut self) -> &mut Message {
        let bytes = self as *mut Self as *mut u8;
        unsafe { std::slice::from_raw_parts_mut(bytes, MESSAGE_SIZE) }
    }
}
