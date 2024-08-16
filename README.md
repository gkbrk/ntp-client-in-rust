# NTP Client in Rust

This project is an implementation of a simple NTP (Network Time Protocol) client in Rust. It demonstrates how to request the current time from an NTP server and adjust the system clock accordingly.

## Features

- Sends a request to an NTP server from the `pool.ntp.org` pool.
- Receives and parses the timestamp from the NTP server response.
- Adjusts the system clock by the received time difference.

## Usage

To use the NTP client, ensure you have Rust installed. Clone the repository and run the project using Cargo:

```sh
git clone https://github.com/gkbrk/ntp-client-in-rust.git
cd ntp-client-in-rust
cargo build --release
sudo ./target/release/ntp-client-in-rust
```

## Code Explanation

- **Random Number Generator (Rng):** Initializes a simple random number generator using the current time in milliseconds.
- **NTP Request and Response:** Creates a UDP socket to send and receive data to/from the NTP server.
- **Time Adjustment:** Parses the received NTP packet and calculates the time difference to adjust the system clock.

## Important Notes

- The program sets an alarm for 5 seconds to timeout if it cannot synchronize the time within that period.
- Adjusting the system clock typically requires root privileges. Ensure you run the program with the necessary permissions.
- The current implementation uses `libc::adjtime` to adjust the system clock, which may not be supported on all platforms.
