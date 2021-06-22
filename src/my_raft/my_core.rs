use log::info;
use math::round;
use rand::Rng;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
extern crate log;
extern crate simplelog;
use crate::my_raft::my_types::{Leader, log_entry, RPC_client, Servers, Vote_response, Vote_request, State, Peer};

pub(crate) fn start_the_server(server:Arc<Mutex<Servers>>, Rpc_client:impl RpcClient + std::marker::Send + 'static){
    //server.lock().unwrap().start_severs();
    server.lock().unwrap().start_severs();
    //server.start_severs();//start timing
    let background_handle_task=thread::spawn(move||{
        loop {
            timeout_handle(Arc::clone(&server),&Rpc_client);
            heartbeat_broadcast(Arc::clone(&server),&Rpc_client);
        }
    });
    background_handle_task.join().unwrap();

}
fn handle_the_vote_request(server:Arc<Mutex<Servers>>,vote_request:Vote_request)->Vote_response{
    let mut server=server.lock().unwrap();
    match server.voted_for{
        Some(_)=>Vote_response{
            term:vote_request.term,
            vote_granted:false
        },
        None=>{
            if vote_request.term>server.term{
                server.voted_for=Some(
                    Peer{
                        id:vote_request.candidate_id,
                        address: SocketAddrV4::new(Ipv4Addr::LOCALHOST, 7879),
                    });
                    Vote_response{
                        term:vote_request.term,
                        vote_granted:true
                    }
                }else{
                    Vote_response {
                        term: request.term,
                        vote_granted: false,
                    }
                }
            }
        }
}

fn heartbeat_broadcast(server:Arc<Mutex<Servers>>,RPC_client:impl RPC_client){
    let leader_current=server.State==State::Leader;
    if leader_current{
        let term=server.term;
        let id=server.id;
        RPC_client.broadcast_log_entry(log_entry::Heartbeat{term:term,peer_id:id});

    }
    let mut rng=rand::thread_rng();
    thread::sleep(Duration::new(rng.gen_range(1..7), 0));//randomly sleep a while
}
//server whether need use Arc?
fn timeout_handle(server:Arc<Mutex<Servers>>, RPC_client:&impl RPC_client){
    let id_server=server.lock().unwrap().id.to_string();
    //let id_server=server.id.to_string();
    let whether_timeout=server.has_timed_out();
    if whether_timeout{
        info!{"Servers timeout {}",id_server}
        new_elect(Servers,RPC_client);
    }
}
fn new_elect(server:Arc<Mutex<Servers>>,RPC_client:&impl RPC_client){
    let vote_request=prepare_the_request(Arc::clone(&server));
    // let mut server=server;
    let server_id=server.lock().unwrap().id.to_string();
    let current_term=server.lock().unwrap().term;
    info!{"server{} in {},start the new election",server_id,current_term};
    let response =match vote_request{
        Some(request)=>Some(RPC_client.request_vote(request)),
        None=>None
    };
    if let Some(r) = response {
        let own_election;
        own_election=win_election(server,r)&&!server.has_timed_out();
        if own_election{
            be_leader(Arc::clone(&server),RPC_client);
        }
    }
}
fn prepare_the_request(server:Arc<Mutex<Servers>>)->Option<Vote_request>{
    if server.lock().unwrap().State==State::LEADER{
        return None
    }
    let mut server_temp=server.lock().unwrap();
    server_temp.State=State::Candidate;
    server_temp.term=server_temp.term+1;
    server_temp.refresh_timeout();
    server_temp.voted_for=Some(Peer{
        id:server_temp.id.to_string(),
        address:server_temp.address
    });
    let new_term=server.term;
    let id=server.id.to_string();
    Some(Vote_request{
        term:new_term,
        candidate_id:id
    })
    
}
fn win_election(server:Arc<Mutex<Servers>>,response:Vec<Vote_response>)->bool{
    let number=server.number_of_peers+1;
    let vote=response.iter().filter(|r|r.vote_granted.count());
    let mini=round::floor((number/2)as f64  , 0);
    (vote+1)>mini as usize&&State::Candidate==server.lock().unwrap().State
}
fn be_leader(server:Arc<Mutex<Servers>>,RPC_client:impl RPC_client){
    let mut server=server.lock().unwrap();
    server.become_leader();
    let log_entry=log_entry::Heartbeat{
        term:server.term,
        peer_id:server.id.to_string(),
    };
    RPC_client.broadcast_log_entry(log_entry);
}