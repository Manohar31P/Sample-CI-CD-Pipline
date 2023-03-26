# port-scanner
Port Scanner made using Rust

Download the latest binaries from [Releases](https://github.com/HarshitRuwali/port-scanner/releases). <br>
Works on Linux, Windows and MacOS.


Usage:
- Binary:
    - Scan for first 1000 ports:
        `port-scanner -t thread_count ip`

    - Scan for all open ports:
        `port-scanner -t thread_count ip -p-`

- dev:
    `cargo run -- -t 100 192.168.1.1`


Build:

`cargo build --release`
