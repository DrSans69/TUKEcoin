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
use tokio::{self, sync::mpsc};

use crate::app::Action;

const MAX_IDLE: u64 = 60;

macro_rules! netprint {
    ($sender:expr, $($arg:tt)*) => {
        let _ = $sender.send(Action::NetworkMessage(format!($($arg)*)));
    };
}

fn handle_swarm_event(
    event: SwarmEvent<FloodsubEvent>,
    swarm: &mut Swarm<Floodsub>,
    sender: &mpsc::UnboundedSender<Action>,
) {
    match event {
        SwarmEvent::Behaviour(FloodsubEvent::Message(m)) => {
            netprint!(
                sender,
                "📨 Received: '{}' from {:?}",
                String::from_utf8_lossy(&m.data),
                m.source
            );
        }
        SwarmEvent::ConnectionEstablished { peer_id, .. } => {
            netprint!(sender, "✅ Connected to {:?}", peer_id);
            swarm
                .behaviour_mut()
                .add_node_to_partial_view(peer_id.clone());
        }
        SwarmEvent::ConnectionClosed { peer_id, .. } => {
            netprint!(sender, "❌ Closed {:?}", peer_id);
            swarm
                .behaviour_mut()
                .add_node_to_partial_view(peer_id.clone());
        }
        other => {
            netprint!(sender, "🌀 Other: {:?}", other);
        }
    }
}

fn create_swarm() -> Swarm<Floodsub> {
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());

    let transport = tcp::tokio::Transport::default()
        .upgrade(upgrade::Version::V1)
        .authenticate(plaintext::Config::new(&id_keys))
        .multiplex(yamux::Config::default())
        .boxed();

    let behaviour = Floodsub::new(peer_id);

    let config: Config =
        Config::with_tokio_executor().with_idle_connection_timeout(Duration::from_secs(MAX_IDLE));

    Swarm::new(transport, behaviour, peer_id, config)
}

pub async fn run_server(
    sender: mpsc::UnboundedSender<Action>,
) -> Result<(), Box<dyn std::error::Error>> {
    let listen_addr: Multiaddr = "/ip4/0.0.0.0/tcp/6969".parse()?;

    let mut swarm = create_swarm();
    netprint!(sender, "Peer id - {}", swarm.local_peer_id());

    swarm.listen_on(listen_addr)?;

    let topic = Topic::new("chat");
    swarm.behaviour_mut().subscribe(topic.clone());

    loop {
        if let Some(event) = swarm.next().await {
            handle_swarm_event(event, &mut swarm, &sender);
        }
    }
}

pub async fn run_client(
    sender: mpsc::UnboundedSender<Action>,
) -> Result<(), Box<dyn std::error::Error>> {
    let listen_addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse()?;

    let mut swarm = create_swarm();
    netprint!(sender, "Peer id - {}", swarm.local_peer_id());

    swarm.listen_on(listen_addr)?;

    let topic = Topic::new("chat");
    swarm.behaviour_mut().subscribe(topic.clone());

    let remote_addr: Multiaddr = "/ip4/192.168.178.118/tcp/6969".parse()?;

    swarm.dial(remote_addr.clone())?;
    netprint!(sender, "Dialed {}", remote_addr);

    loop {
        if let Some(event) = swarm.next().await {
            handle_swarm_event(event, &mut swarm, &sender);
        }
    }
}
