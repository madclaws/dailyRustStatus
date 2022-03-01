#[async_std::main]
fn main() {
    println!("Chat P2P!");
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());


    // How to send data
    let transport = libp2p::development_transport(local_key).await?;


    let floodsub_topic = floodsub::Topic::new("chat");


    #[derive(NetworkBehaviour)]
    #[behaviour(out_event = "OutEvent")]
    struct MyBehaviour {
        floodsub: Floodsub,
        mdns: Mdns,

        #[behaviour(ignore)]
        #[allow(dead_code)]
        ignored_member: bool,
    }

    let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));

    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
    }
}