cd server || exit
cargo run &
cd ../client || exit
cargo run -- ../../1.pcap 
