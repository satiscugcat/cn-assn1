use std::{
    net::TcpListener,
    io::{Read, Write},
    fs::File,
};

fn parse_dns_name(data: &[u8]) -> String {
    let mut pos = 0;
    let mut labels = Vec::new();
    while pos < data.len() {
        let len = data[pos] as usize;
        if len == 0 {
            break;
        }
        pos += 1;
        if pos + len > data.len() {
            break;
        }
        labels.push(String::from_utf8_lossy(&data[pos..pos+len]).to_string());
        pos += len;
    }
    labels.join(".")
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut f = File::create("foo.csv")?;
	writeln!(
        f,
        "Custom header value (HHMMSSID),Domain name,Resolved IP address"
    )?;

    for stream in listener.incoming() {
	let mut buf = [0u8; 512];
        let mut stream = stream.unwrap();
        let message_size = stream.read(&mut buf)?;
	// let dns_query_name = &buf[62 .. message_size.saturating_sub(4)];
	// let dns_query_name = match str::from_utf8(dns_query_name) {
    //         Ok(v) => v,
    //         Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
	// };
	let dns_query_name = parse_dns_name(&buf[62..message_size.saturating_sub(4)]);
	let hours = 10 * buf[0] + buf[1];
	let minutes = 10 * buf[2] + buf[3];
    let seconds = 10 * buf[4] + buf[5];
    let id = 10 * buf[6] + buf[7];
	let ip_pool_start =
	if (12..18).contains(&hours) {
		5
	} else if hours >= 20 || hours < 4 {
		10
	} else {
		0
	};

	let final_ip = format!("192.168.1.{}", ip_pool_start + (id % 5));

	// f.write(format!("{}{}{}{}{}{}{}{}",buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7]).as_bytes()) ?;
	// f.write(",".as_bytes())?;
	// f.write(dns_query_name.as_bytes())?;
	// f.write(",".as_bytes())?;
	// f.write(final_ip.as_bytes())?;
	// f.write("\n".as_bytes())?;
	writeln!(
		f,
		"{:02}{:02}{:02}{:02},{},{}",
		hours, minutes, seconds,id, dns_query_name, final_ip
	)?;	
	
    }
    f.flush()?;

    
    Ok(())
}