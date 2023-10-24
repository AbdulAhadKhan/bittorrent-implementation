// TODO: remove allow dead_code

#[derive(Debug)]
#[allow(dead_code)]
pub struct Torrent {
    announce: String,
    info: Info,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Info {
    length: u32,
    name: String,
    piece_length: u32,
    pieces: String,
}

impl Torrent {
    pub fn new(path: &str) -> Torrent {
        println!("{:?}", path);
        return Torrent {
            announce: "Hello".to_string(),
            info: Info {
                length: 3,
                name: "World".to_string(),
                piece_length: 32,
                pieces: "hash".to_string(),
            },
        };
    }
}
