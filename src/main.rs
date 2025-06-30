use std::time::Duration;

use libp2p::{
    self,
    core::upgrade,
    floodsub::{Floodsub, FloodsubEvent, Topic},
    futures::StreamExt,
    identity, plaintext,
    swarm::{Config, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId, Swarm, Transport,
};
use tokio::{
    self,
    io::{AsyncBufReadExt, BufReader},
};

const MAX_IDLE: u64 = 60;

#[tokio::main]
async fn main() {
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer id: {:?}", peer_id);

    let transport = tcp::tokio::Transport::default()
        .upgrade(upgrade::Version::V1)
        .authenticate(plaintext::Config::new(&id_keys))
        .multiplex(yamux::Config::default())
        .boxed();

    let behaviour = Floodsub::new(peer_id);

    let config: Config =
        Config::with_tokio_executor().with_idle_connection_timeout(Duration::from_secs(MAX_IDLE));

    let mut swarm = Swarm::new(transport, behaviour, peer_id, config);

    //

    let listen_addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();

    swarm.listen_on(listen_addr).unwrap();

    let topic = Topic::new("chat");

    swarm.behaviour_mut().subscribe(topic.clone());

    let remote_addr: Multiaddr = "/ip4/192.168.137.6/tcp/57023".parse().unwrap();

    swarm.dial(remote_addr.clone()).unwrap();
    println!("Dialed {}", remote_addr);

    let mut stdin = BufReader::new(tokio::io::stdin()).lines();
    loop {
        tokio::select! {
            line = stdin.next_line() => {
                if let Ok(Some(line)) = line {
                    swarm.behaviour_mut().publish(topic.clone(), line.into_bytes());
                } else {
                    break;
                }
            }
            event = swarm.next() => {
                match event {
                    Some(SwarmEvent::Behaviour(FloodsubEvent::Message(m))) => {
                        println!(
                            "📨 Received: '{}' from {:?}",
                            String::from_utf8_lossy(&m.data),
                            m.source
                        );
                    }
                    Some(SwarmEvent::ConnectionEstablished { peer_id, .. }) => {
                        println!("✅ Connected to {:?}", peer_id);
                        swarm.behaviour_mut().add_node_to_partial_view(peer_id.clone());
                    }
                    Some(other) => {
                        println!("🌀 Other: {:?}", other);
                    }
                    None => break,
                }
            }
        }
    }
}
