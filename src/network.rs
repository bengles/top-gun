use super::*;
use laminar::{Packet, Socket, SocketEvent};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

pub const SERVER: &str = "0.0.0.0:12351";
const CLIENT: &str = "0.0.0.0:12352";

pub fn receive_thread(
    receiver: Receiver<SocketEvent>,
    arc: Arc<Mutex<Vec<NetworkMessage>>>,
    ips: Arc<Mutex<std::collections::HashSet<SocketAddr>>>,
    ip: SocketAddr,
) {
    println!("Listening on {:?}", ip);
    loop {
        if let Ok(event) = receiver.recv() {
            let mut messages = arc.lock().unwrap();
            match event {
                SocketEvent::Packet(packet) => {
                    let mut ip_lock = ips.lock().unwrap();
                    let msg = packet.payload();
                    ip_lock.insert(packet.addr());
                    if let Ok(message) = serde_json::from_str(&String::from_utf8_lossy(msg)) {
                        messages.push(message);
                    }
                }
                SocketEvent::Timeout(_) => {
                    println!("TIMEOUT");
                }
                _ => {}
            }
        }
    }
}

pub fn send_events_to_clients(
    sender: &mut Sender<Packet>,
    send_queue: &mut Vec<NetworkMessage>,
    ips: &mut Arc<Mutex<HashSet<SocketAddr>>>,
) {
    let ips_lock = ips.lock().unwrap();
    for message in send_queue.iter() {
        for ip in &*ips_lock {
            let _ = sender
                .try_send(Packet::reliable_unordered(
                    *ip,
                    serde_json::to_vec(&message).unwrap(),
                ))
                .expect("This should send");
        }
    }
    send_queue.clear();
}

pub fn send_events_to_ip(
    sender: &mut Sender<Packet>,
    send_queue: &mut Vec<NetworkMessage>,
    ip: SocketAddr,
) {
    for message in send_queue.iter() {
        let _ = sender
            .try_send(Packet::reliable_unordered(
                ip,
                serde_json::to_vec(message).unwrap(),
            ))
            .expect("This should send");
    }
    send_queue.clear();
}

pub struct NetworkContext {
    pub is_host: bool,
    pub frequency: f32,
    pub send_queue: Vec<NetworkMessage>,
    pub ips_lock: Arc<Mutex<HashSet<SocketAddr>>>,
    pub received_messages_lock: Arc<Mutex<Vec<NetworkMessage>>>,
    pub sender: Sender<Packet>,
}

impl NetworkContext {
    pub const FREQUENCY: f32 = 1.0 / 30.0;

    pub fn new(is_host: bool) -> NetworkContext {
        let mut socket = if is_host {
            // host
            Socket::bind(SERVER).unwrap()
        } else {
            // client
            let addr: SocketAddr = CLIENT.parse().unwrap();
            Socket::bind(addr).unwrap()
        };

        let ip = socket.local_addr().unwrap().clone();
        let send_queue: Vec<NetworkMessage> = vec![];
        let ips_lock: Arc<Mutex<HashSet<SocketAddr>>> =
            Arc::new(Mutex::new(HashSet::<SocketAddr>::new()));
        let received_messages_lock: Arc<Mutex<Vec<NetworkMessage>>> =
            Arc::new(Mutex::new(Vec::<NetworkMessage>::new()));
        let received_messages_lock_receive_thread = received_messages_lock.clone();
        let receive_thread_ips = ips_lock.clone();
        let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
        let _thread = thread::spawn(move || socket.start_polling());
        let _receive_thread = thread::spawn(move || {
            network::receive_thread(
                receiver,
                received_messages_lock_receive_thread,
                receive_thread_ips,
                ip,
            )
        });

        NetworkContext {
            is_host: is_host,
            frequency: NetworkContext::FREQUENCY,
            send_queue: send_queue,
            ips_lock: ips_lock,
            received_messages_lock: received_messages_lock,
            sender: sender,
        }
    }
}
