use std::{
    net::TcpListener,
    io::{Read, Write},
    fs::File,
};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut f = File::create("foo.csv")?;
    for stream in listener.incoming() {
	let mut buf: &mut [u8] = &mut [0; 512];
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

	f.write(format!("{}{}{}{}{}{}{}{}",buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7]).as_bytes()) ?;
	f.write(",".as_bytes())?;
	f.write(dns_query_name.as_bytes())?;
	f.write(",".as_bytes())?;
	f.write(final_ip.as_bytes())?;
	f.write("\n".as_bytes())?;
    }
    f.flush()?;

    
    Ok(())
}
