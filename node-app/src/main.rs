use libp2p::{
    futures::StreamExt,
    identity,
    //mdns,
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    swarm::{Swarm, SwarmEvent},
    PeerId,
};
use std::error::Error;

static POD_PORT: &str = "4242";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());

    let transport = libp2p::development_transport(id_keys).await?;
    let behaviour = Mdns::new(MdnsConfig::default()).await?;
    let mut swarm = Swarm::new(transport, behaviour, peer_id);
    let mut multi_address_pod: String = "/ip4/0.0.0.0/tcp/".to_owned();

    multi_address_pod.push_str(&POD_PORT);

    println!("Local peer id: {:?}", peer_id);
    println!("Node a Multiaddress: {}", multi_address_pod);
    swarm.listen_on(multi_address_pod.parse()?)?;
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on local address {:?}", address)
            }
            SwarmEvent::Behaviour(MdnsEvent::Discovered(peers)) => {
                for (peer, addr) in peers {
                    println!("discovered {} {}", peer, addr);
                }
            }
            SwarmEvent::Behaviour(MdnsEvent::Expired(expired)) => {
                for (peer, addr) in expired {
                    println!("expired {} {}", peer, addr);
                }
            }
            _ => {}
        }
    }
}