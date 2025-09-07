use std::env;
use std::fs::File;
use std::net::TcpStream;
use pcap_file::pcap::*;
use std::io::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 || args.len() == 1 {
	panic!("This program takes only one .pcap file as input.")
    }
    let file = File::open(&args[1]).expect("Could not find file.");
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    let mut pcap_reader = PcapReader::new(file).unwrap();

    // Read argument
    while let Some(pkt) = pcap_reader.next_packet() {
	//Check if there is no error
	let pkt = pkt.unwrap();

	let data = pkt.data;
	// Filtering out DNS queries.
	if is_query(&data) {continue};

	
	stream.write(&data)?;
    }
    
    Ok(())
}


fn is_query(packet: &[u8]) -> bool {
    packet[2] < 128
}
