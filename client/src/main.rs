use std::fs::File;
use pcap_file::pcap::{PcapReader};
use std::env;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_in = File::open(&args[1]).expect("Error opening PCAP file.");
    let mut pcap_reader = PcapReader::new(file_in).unwrap();
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    
    while let Some(pkt) = pcap_reader.next_packet() {
	//Check if there is no error
	
	let pkt = pkt.unwrap();
	let data = pkt.data;
	if is_dns_query(&data) {
	    continue;
	} else {
	    todo!()
	}
    }


    Ok(())
}
/// This function checks whether a packet is a DNS query. By RFC 1035, the first bit of the
/// third octect of a message determines this (0 if query, 1 if not). Because of network
/// byte order, we thus check the highest bit of the third octet.
fn is_dns_query(data: &[u8]) -> bool {
    data[2] < 128
}
