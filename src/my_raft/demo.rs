use crate::my_raft::my_tcp_rcp::{Tcp_Rcp_server, Tcp_Rpc_client};
use log::info;
use rand::{Rng, thread_rng};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use crate::my_raft::my_types::{Servers, Peer};
use crate::my_raft::my_core;

pub fn start_demo(){
    let mut Rpc_server:Vec<Tcp_Rcp_server>=Vec::new();
    let mut thread_rng=thread_rng();
    let address=SocketAddrV4::new(Ipv4Addr::LOCALHOST,3300);
    let server1=Arc::new(Mutex::new(Servers::new(
        Duration::new(thread_rng.gen_range(2..5),0),
        2,
                    address,
                    "server1".to_string()
    )));
    let address1_peer=vec![
        Peer{
            id:"server2".to_string(),
            address:SocketAddrV4::new(Ipv4Addr::LOCALHOST,3301)
        },
        Peer{
            id:"server3".to_string(),
            address: SocketAddrV4::new(Ipv4Addr::LOCALHOST,3302)
        }
    ];
    Rpc_server.push(Tcp_Rcp_server::new(Arc::clone(&server1),address));
    let address2=SocketAddrV4::new(Ipv4Addr::LOCALHOST,3301);
    let server2=Arc::new(Mutex::new(Servers::new(
        Duration::new(thread_rng.gen_range(3..6),0),
        2,
        address2,
        "server2".to_string()
    )));
    let address2_peer=vec![
        Peer {
            id: "server_1".to_string(),
            address: SocketAddrV4::new(Ipv4Addr::LOCALHOST, 3300),
        },
        Peer {
            id: "server_3".to_string(),
            address: SocketAddrV4::new(Ipv4Addr::LOCALHOST, 3302),
        },
    ];
    Rpc_server.push(Tcp_Rcp_server::new(Arc::clone(&server2),address2));

    let address3=SocketAddrV4::new(Ipv4Addr::LOCALHOST,3302);
    let server3=Arc::new(Mutex::new(Servers::new(
        Duration::new(thread_rng.gen_range(4..7),0),
        2,
        address3,
        "server3".to_string()
    )));
    let address3_peer=vec![
        Peer {
            id: "server_1".to_string(),
            address: SocketAddrV4::new(Ipv4Addr::LOCALHOST, 3300),
        },
        Peer {
            id: "server_2".to_string(),
            address: SocketAddrV4::new(Ipv4Addr::LOCALHOST, 3301),
        },
    ];
    Rpc_server.push(Tcp_Rcp_server::new(Arc::clone(&server3),address3));

    let mut server_threads=Vec::new();
    for Rpc_server in Rpc_server {
        server_threads.push(thread::spawn(move||{
            Rpc_server.start_server()}))
    }
    thread::sleep(Duration::new(1,0));
    let mut raft_thread= Vec::new();
    raft_thread.push(thread::spawn(move||{
        let Tcp_client=Tcp_Rpc_client::new(&address1_peer);
        {
            let tmp_server=server1.lock().unwrap();
            info!("the server{},has a timeout of {}",tmp_server.id,tmp_server.timeout.as_secs())
        }
        my_core::start_the_server(Arc::clone(&server1),Tcp_client);
    }));
    for server_thread in server_threads {
        server_thread.join().unwrap()
    }
    for thread in raft_thread {
        thread.join().unwrap();
    }
}