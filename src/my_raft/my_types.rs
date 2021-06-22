use log::info;
use serde::{Deserialize, Serialize};
use std::net::SocketAddrV4;
use std::time::{Duration, Instant};
#[derive(Debug, PartialEq)]
pub enum State{
    Leader,
    Candidate,
    Follower,
}
pub struct Clients{

}
#[derive(Debug)]
pub struct Servers{
    pub(crate) State:State,
    pub(crate) id:String,
    pub(crate) address: SocketAddrV4,
    pub(crate) timeout: Duration,
    pub(crate) term: u64,
    current_leader: Option<Leader>,
    pub(crate) number_of_peers: usize,
    pub(crate) voted_for:Option<Peer>,
    log_entry:Vec<log_entry>,
    next_timeout:Option<Instant>
    //just for setting the time to run raft, actually  it is a trigger thing
}
impl Servers{
    pub fn new(
        timeout:Duration,
        number_of_peers:usize,
        address:SocketAddrV4,
        id: String,
    )->Self{
        Servers{
            log_entry:Vec::new(),
            State:State::Follower,
            id:id,
            timeout:timeout,
            number_of_peers:number_of_peers,
            address:address,
            term:0,
            current_leader:None,
            voted_for:None,
            next_timeout:None
        }
    }
    pub fn refresh_timeout(self:&mut Self){
        self.next_timeout=Some(Instant::now()+self.timeout);
    }
    pub fn become_leader(self:&mut Self){
        if self.State==State::Candidate{
            info!{"Severs{} has been selected ! in the  term{}",self.id,self.term};
        };
        self.State=State::Leader;
        self.next_timeout=None;
    }
    pub fn start_severs(&mut self){
        self.refresh_timeout();
        //just reset the time
    }
    pub fn has_timed_out(&mut self)->bool{
        match self.next_timeout{
            Some(t)=>Instant::now()>t,
            None=>false
        }
    }
}
// struct Time{
    
// }
#[derive(Debug, Serialize,Deserialize)]
pub struct Leader{
    id:String,
    term:u64,
}
#[derive(Debug, Serialize,Deserialize)]
pub struct Peer{
    pub(crate) id:String,
    pub(crate) address:SocketAddrV4
}
pub struct Vote_request{
    pub(crate) term:u64,
    pub(crate) candidate_id:String,
}
pub struct Vote_response{
    pub(crate) term:u64,
    pub(crate) vote_granted:bool,
}
#[derive(Debug)]
pub enum log_entry{
    Heartbeat{
        term:u64,
        peer_id:String
    }
}

pub trait RPC_client{
//only leader can use it  
    fn request_vote(&self,request:Vote_request)->Vec<Vote_response>;
    fn broadcast_log_entry(&self,log_entry:log_entry);
}