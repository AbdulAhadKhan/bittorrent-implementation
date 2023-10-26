use tokio::net::TcpStream;

#[allow(unused)]
#[derive(Debug)]
pub struct Handshake<'a> {
    reserved: u64,
    info_hash: &'a [u8; 20],
    peer_id: &'a [u8; 20],
}

#[allow(unused)]
impl Handshake<'_> {
    const LENGTH: u8 = 19;
    const PROTOCOL_STRING: &[u8; 19] = b"BitTorrent protocol";

    pub fn new<'a>(info_hash: &'a [u8; 20], peer_id: &'a str) -> Handshake<'a> {
        let peer_id: &[u8; 20] = peer_id.as_bytes().try_into().unwrap();

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
