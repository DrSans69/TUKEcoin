use tokio::time::Duration;
use libp2p::{
    core::{transport::{upgrade, Transport}},
    futures::channel::mpsc,
    mplex,
    noise::{
        Keypair as NoiseKeypair, NoiseConfig, X25519Spec                  // The Noise protocol config
    },
    swarm::{SwarmBuilder},
    tcp::TokioTcpConfig,
    Swarm,
};
use futures::stream::StreamExt;
use log::{info, error};
use tokio::io::{AsyncBufReadExt, stdin};
use tokio::spawn;
use tokio::time::sleep;
use tokio::select;

mod blockchain;
mod p2p;


#[tokio::main]
async fn main(){
    pretty_env_logger::init();

    info!("Peer ID: {}", p2p::PEER_ID.clone());
    let (response_sender, mut response_rcv) = mpsc::unbounded();
    let (init_sender, mut init_rcv) = mpsc::unbounded();

    let auth_keys = NoiseKeypair::<X25519Spec>::new()
        .into_authentic(&*p2p::KEYS)
        .expect("can create noise keys");

    let transp = TokioTcpConfig::new()
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(auth_keys).into_authenticated())
        .multiplex(mplex::MplexConfig::new())
        .boxed();

    let behaviour = p2p::AppBehaviour::new(blockchain::App::new(), response_sender, init_sender.clone()).await;

    let mut swarm = SwarmBuilder::new(transp, behaviour, *p2p::PEER_ID).executor(Box::new(|fut|{spawn(fut);})).build();
    
    let mut stdin = tokio::io::BufReader::new(stdin()).lines();

    Swarm::listen_on(
        &mut swarm,
        "/ip4/0.0.0.0/tcp/0"
            .parse()
            .expect("can get a local socket"),
    )
    .expect("swarm can't be started");

    spawn(async move {
        sleep(Duration::from_secs(1)).await;
        info!("sending init event!");
        init_sender.unbounded_send(p2p::EventType::Init).expect("can send init event");
    }); 

    loop {
        let evt = {
            select! {
                // Case 1: Waiting for user input from stdin
                line = stdin.next_line() => Some(p2p::EventType::Input(line.expect("can get line").expect("can read line from stdin"))),
                // Case 2: Receiving a response from a channel
                response = response_rcv.next() => {
                    Some(p2p::EventType::LocalChainResponse(response.expect("response exists")))
                },
                // Case 3: Receiving an "init" signal (e.g. from a setup process)
                _init = init_rcv.next() => {
                    Some(p2p::EventType::Init)
                }
                // Case 4: A swarm/networking event happened (but we're not handling it)
                _event = swarm.select_next_some() => {
                    // info!("Unhandled Swarm Event: {:?}", event);
                    None
                },
            }
        };

        if let Some(event) = evt {
            match event {
                p2p::EventType::Init => {
                    let peers = p2p::get_list_peers(&swarm);
                    swarm.behaviour_mut().app.genesis();
                    info!("connected nodes: {}", peers.len());
                    if !peers.is_empty() {
                        let req = p2p::LocalChainRequest {
                            from_peer_id: peers
                            .iter()
                            .last()
                            .expect("at least one pear")
                            .to_string(),
                        };
                        info!("getting copy of a blockain from peer {}", peers.iter().last().expect("at least one pear"));
                        let json = serde_json::to_string(&req).expect("can jsonify request");
                        swarm
                            .behaviour_mut()
                            .floodsub
                            .publish(p2p::CHAIN_TOPIC.clone(), json.as_bytes());
                    }
                }
                p2p::EventType::LocalChainResponse(resp) => {
                    let json = serde_json::to_string(&resp).expect("can jsonify response");
                    swarm
                        .behaviour_mut()
                        .floodsub
                        .publish(p2p::CHAIN_TOPIC.clone(), json.as_bytes());
                }
                p2p::EventType::Input(line) => match line.as_str() {
                    "ls p" => p2p::handle_print_peers(&swarm),
                    cmd if cmd.starts_with("ls c") => p2p::handle_print_chain(&swarm),
                    cmd if cmd.starts_with("create b") => p2p::handle_create_block(cmd, &mut swarm),
                    _ => error!("unknown command"),
                },
            }
        }
    }
}
    