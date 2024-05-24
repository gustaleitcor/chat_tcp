use std::net::SocketAddr;

pub struct Message {
    pub message: String,
    pub adresser: SocketAddr,
}
