use super::*;

pub struct NetworkContext {
    pub is_host: bool,
    pub receiver: Receiver<SocketEvent>,
    pub sender: Sender<Packet>,
    pub client_sockets: Vec<std::net::SocketAddr>,
}