use pcap_file::pcap::PcapReader;
use std::env;
use std::fs::File;
use std::io::Write;
use std::net::TcpStream;
use std::time::SystemTime;
 
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 || args.len() == 1 {
        panic!("This program takes in the path of one pcap file as an argument.")
    }
    let file_in = File::open(&args[1]).expect("Error opening PCAP file.");
    let mut pcap_reader = PcapReader::new(file_in).unwrap();
    
    let mut id:u8 = 0;
    while let Some(pkt) = pcap_reader.next_packet() {
        //Check if there is no error

        let pkt = pkt.unwrap();
        let old_data: Vec<u8> = pkt.data.into();
        if !is_dns_query(&old_data) {
            continue;
        } else {
            let (hours, minutes, seconds) = give_current_time();
            let mut new_data = vec![
                hours / 10,
                hours % 10,
                minutes / 10,
                minutes % 10,
                seconds / 10,
                seconds % 10,
                id / 10,
                id % 10,
            ];
            new_data.extend_from_slice(&old_data);
	    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
            stream.write_all(&new_data)?;
	    stream.shutdown(std::net::Shutdown::Both)?;
            id = id + 1;
        }
    }

    Ok(())
}

/// First we figure out if the packet is a DNS packet. We check if the dst port is 53,
/// because in both TCP and UDP, the destination port bytes are at 36-37.
/// By RFC 1035, the first bit of the
/// third octect of a message determines this (0 if query, 1 if not). Because of network
/// byte order, we thus check the highest bit of the third octet.
fn is_dns_query(data: &[u8]) -> bool {
    data.len() > 45 && data[36] == 0 && data[37] == 53 && // checking if the port is 53
	data[44] < 128 // checking if it is a query
}

fn give_current_time() -> (u8, u8, u8) {
    let unix_epoch_age = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("System Time Error")
        .as_secs();
    let seconds_today = unix_epoch_age % (24 * 60 * 60);
    let hours = (seconds_today / 3600) as u8;
    let minutes = ((seconds_today % 3600) / 60) as u8;
    let seconds = (seconds_today % 60) as u8;
    (hours, minutes, seconds)
}