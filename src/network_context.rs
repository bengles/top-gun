use super::*;

pub struct NetworkContext {
    pub is_host: bool,
    pub received_messages: Arc<Mutex<Vec<NetworkMessage>>>,
    pub sender: Sender<Packet>,
    pub client_ips: Arc<Mutex<std::collections::HashSet<std::net::SocketAddr>>>,
    pub server_addr: std::net::SocketAddr,
}

pub fn receive_thread(receiver: Receiver<SocketEvent>, arc: Arc<Mutex<Vec<NetworkMessage>>>, ips: Arc<Mutex<std::collections::HashSet<SocketAddr>>>) {
    loop {
        if let Ok(event) = receiver.recv() {
            println!("{:?}", event);
            let mut messages = arc.lock().unwrap();
            match event {
                SocketEvent::Packet(packet) => {
                    let mut ip_lock = ips.lock().unwrap();
                    let msg = packet.payload();
                    ip_lock.insert(packet.addr());
                    messages.push(serde_json::from_str(&String::from_utf8_lossy(msg)).unwrap());
                }
                SocketEvent::Timeout(_) => {}
                _ => {}
            }
        }
    }
}
