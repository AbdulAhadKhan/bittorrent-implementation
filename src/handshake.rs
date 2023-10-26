use tokio::net::TcpStream;

#[allow(unused)]
pub struct Handshake<'a> {
    reserved: u64,
    info_hash: &'a [u8; 20],
    peer_id: &'a [u8; 20],
}

#[allow(unused)]
impl Handshake<'_> {
    const LENGTH: u8 = 19;
    const PROTOCOL_STRING: &[u8; 19] = b"BitTorrent protocol";

    pub fn new<'a>(info_hash: &'a [u8; 20], peer_id: &'a [u8; 20]) -> Handshake<'a> {
        Handshake {
            reserved: 0,
            info_hash,
            peer_id,
        }
    }

    pub async fn connect(&self, address: &str) {
        let stream = TcpStream::connect(address).await.unwrap();
        println!("{:?}", Handshake::PROTOCOL_STRING.len());
    }
}
