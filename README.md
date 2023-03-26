## Sleep on LAN
This utility allows a server to be run, which will listen on a user-configured port for a packet of 2048 bytes (consisting of random binary data). The utility, upon recieving the correct packet, will attempt to shut down the machine on which the server is running.

### Usage instructions
1. Install cargo using your system's package manager.
2. Clone this repository to a temporary location on your server machine
3. Change directories into the cloned repository, and run `cargo build --release` 
4. Copy the binary in `target/release/sleeponlan` and the `generate_keys.py` script to a permanent location on your machine.
5. Run `generate_keys.py` in the directory that your server binary is in. This will create and generate the two keys that are required.
6. Your server binary can now run. The program uses the RUST_LOG environment variable to change log levels. Levels can be: DEBUG, INFO, or ERROR.