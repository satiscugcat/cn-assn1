use std::{
    net::TcpListener,
    io::{Read, Write},
    fs::File,
};

struct Record <'a>{
    pub custom_header : [u8; 8],
    pub domain_name : &'a str,
    pub ip: (u8, u8, u8, u8)
}
fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut f = File::create("foo.csv")?;
    for stream in listener.incoming() {
	let mut buf: &mut [u8] = &mut [];
        let mut stream = stream.unwrap();
        let message_size = stream.read(&mut buf)?;
	let dns_query_name = &buf[62 .. message_size - 4];
	let dns_query_name = match str::from_utf8(dns_query_name) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
	};
	let hours = 10 * buf[0] + buf[1];
	let id: u8 = 10 * buf[6] + buf[7];
	let ip_pool_start: u8 =
	if 12 <= hours && hours < 18 {
	   5
	} else if 20 <= hours || hours < 4 {
	    10
	} else {
	    0
	};

	let final_ip = format!("{}.{}.{}.{}",192, 168, 1, ip_pool_start + (id%5));

	f.write(&buf[0..7])?;
	f.write(&[46])?;
	f.write(dns_query_name.as_bytes())?;
	f.write(&[46])?;
	f.write(final_ip.as_bytes())?;
	f.write(&[10])?;
    }
    f.flush()?;

    
    Ok(())
}
