use super::*;
use crossbeam_channel::*;
use laminar::{Packet, SocketEvent};
use std::collections::*;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

const SERVER: &str = "127.0.0.1:12351";
const CLIENT: &str = "127.0.0.1:12352";

pub fn receive_thread(
    receiver: Receiver<SocketEvent>,
    arc: Arc<Mutex<Vec<NetworkMessage>>>,
    ips: Arc<Mutex<std::collections::HashSet<SocketAddr>>>,
) {
    loop {
        if let Ok(event) = receiver.recv() {
            let mut messages = arc.lock().unwrap();
            match event {
                SocketEvent::Packet(packet) => {
                    let mut ip_lock = ips.lock().unwrap();
                    let msg = packet.payload();
                    ip_lock.insert(packet.addr());
                    let message = serde_json::from_str(&String::from_utf8_lossy(msg)).unwrap();
                    messages.push(message);

                    if packet.addr() == SERVER.parse().unwrap() {
                        println!("SERVER SEND: {:?}", message);
                    }
                }
                SocketEvent::Timeout(_) => {}
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
                    serde_json::to_vec(message).unwrap(),
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
    pub send_queue: Vec<NetworkMessage>,
    pub ips_lock: Arc<Mutex<HashSet<SocketAddr>>>,
    pub received_messages_lock: Arc<Mutex<Vec<NetworkMessage>>>,
    pub sender: Sender<Packet>,
}

impl NetworkContext {
    pub fn new(is_host: bool) -> NetworkContext {
        let mut socket = if is_host {
            // host
            Socket::bind(SERVER).unwrap()
        } else {
            // client
            let addr: SocketAddr = CLIENT.parse().unwrap();
            Socket::bind(addr).unwrap()
        };

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
            )
        });

        NetworkContext {
            send_queue: send_queue,
            ips_lock: ips_lock,
            received_messages_lock: received_messages_lock,
            sender: sender,
        }
    }
}
