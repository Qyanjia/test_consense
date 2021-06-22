mod my_raft;
use log::LevelFilter;
use simplelog::{Config, TermLogger, TerminalMode, TermLogError};
fn main() {
    TermLogger::init(LevelFilter,Config::default(),TerminalMode);
    my_raft::demo::start_demo();
    println!("Hello, world!");
}
