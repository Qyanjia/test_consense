use crate::my_raft::my_types::{log_entry, Peer, RPC_client, Vote_request, Vote_response};
use crate::my_raft::my_types::Servers;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::SocketAddrV4;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
#[derive(Serialize, Deserialize, Debug)]
enum Rpc_Message{
    Vote_request{term:u64,candidate_id:String},
    Vote_response{term:u64,vote_granted:bool},
    Heart_beat{term:u64,peer_id:String},
    Heart_beat_response{term:u64,peer_id:String}
}
pub struct Tcp_Rpc_client{
    servers:HashMap<String,TcpStream>
}
pub struct Tcp_Rcp_server{
    server: Servers,
    address: SocketAddrV4
}
impl RPC_client for Tcp_Rpc_client {
    fn request_vote(&self,request:Vote_request)->Vec<Vote_response>{
        let rpc_message=Rpc_Message::Vote_request{
            term:request.term,
            candidate_id:request.candidate_id
        }; //plz others vote for him
        let binary_vote_request=bincode::serialize(&rpc_message).unwrap();
        let mut rpc_responses:Vec<Rpc_Message>=Vec::new();
        for mut stream in self.servers.values(){
            stream.write(&binary_vote_request).unwrap();
            let mut buffers=[0;256];
            stream.read(&mut buffers).unwrap();
            rpc_responses.push(bincode::deserialize(&buffers).unwrap())
        }
        let mut response=Vec::new();
        for rpc_resp in rpc_responses{
            if let Rpc_Message::Vote_response{term,vote_granted} = rpc_resp {
                response.push(Vote_response{
                    term:term,
                    vote_granted:vote_granted
                });
            }
        }
        response 
    }
    fn broadcast_log_entry(&self,log_entry:log_entry){
        if let log_entry::Heartbeat{term,peer_id} = log_entry {
            let rpc_message=Rpc_Message::Heart_beat{
                term:term,
                peer_id:peer_id
            };
            let binary_broadcast=bincode::serialize(&rpc_message).unwrap();
            for mut stream in self.servers.values(){
                stream.write(&binary_broadcast).unwrap();
                let mut buffers=[0;256];
                stream.read(&mut buffers).unwrap();
            }
        }
    }
}
impl Tcp_Rpc_client{
    pub fn new(peers:&Vec<Peer>)->Self{
        let mut server=HashMap::new();
        for peer in peers.iter() {
            let stream=TcpStream::connect(&peer.address).unwrap();
            server.insert(&peer.id, stream);
        }
        Tcp_Rpc_client{
            servers:server
        }
    }
}
impl Tcp_Rcp_server{
    pub fn new(server:Arc<Mutex<Servers>>,address:SocketAddrV4)->Self{
        Tcp_Rcp_server{
            server:server,
            address:address
        }
    }
    pub fn start_server(&self){
        info!("starting the server at{}...",self.address);
        let listener=TcpListener::bind(self.address).unwrap();
        for stream in listener.incoming() {
            let server_clone=self.server;
            match stream{
                Ok(stream)=>{
                    thread::spawn(move||)
                }
                Err(e)=>{
                    info!{"Error while listening to the client{}",e}
                }
            }
        }
    }
}

fn handle_the_connection(server:Servers,mut stream:TcpStream){
    loop{
        let mut buffer=[0;256];
        stream.read(&mut buffer).unwrap();
        //let binary_code=bincode::serialize(value: &T)
        let deserialize_code:Rpc_Message=bincode::deserialize(&buffer).unwrap();
        let response=match deserialize_code{
            Rpc_Message::Heart_beat{term,peer_id}=>{
                handle_the_log_entry()
            }
            Rpc_Message::Vote_request{term,candidate_id}=>{
                handle_the_request()
            }
            _=>Vec::new()
        }
        stream.write(&response).unwrap();
        stream.flush().unwrap();
    }
}
fn handle_the_log_entry(server:Servers,term:u64,peer_id:String)->Vec<()>{

}
fn handle_the_request(server:Servers,term:u64,candidate_id:String)->Vec<>{
    
}