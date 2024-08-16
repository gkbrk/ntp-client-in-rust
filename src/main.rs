// This is free and unencumbered software released into the public domain.

// Anyone is free to copy, modify, publish, use, compile, sell, or
// distribute this software, either in source code form or as a compiled
// binary, for any purpose, commercial or non-commercial, and by any
// means.

// In jurisdictions that recognize copyright laws, the author or authors
// of this software dedicate any and all copyright interest in the
// software to the public domain. We make this dedication for the benefit
// of the public at large and to the detriment of our heirs and
// successors. We intend this dedication to be an overt act of
// relinquishment in perpetuity of all present and future rights to this
// software under copyright law.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.

#[inline(always)]
fn ernd(x: u64, y: u64, k: u64) -> (u64, u64) {
    let mut x = x;
    let mut y = y;
    x = x.rotate_right(8);
    x = x.wrapping_add(y);
    x ^= k;
    y = y.rotate_left(3);
    y ^= x;
    (x, y)
}

fn get_millis() -> u64 {
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    time.as_secs() * 1000 + time.subsec_millis() as u64
}

struct Rng {
    state: u64,
}

impl Rng {
    fn new() -> Self {
        Self {
            state: get_millis(),
        }
    }

    fn next(&mut self) -> u64 {
        self.state += 1;
        let (x, y) = (0, self.state);
        let (x, y) = ernd(x, y, 13785932440550348363);
        let (x, y) = ernd(x, y, 11104037483222624392);
        let (x, y) = ernd(x, y, 10038335852670131637);
        let (x, y) = ernd(x, y, 6571442071688614510);
        let (x, y) = ernd(x, y, 2021207194241289424);
        let (_x, y) = ernd(x, y, 12957456176858273080);
        return y;
    }
}

fn main() {
    let mut rng = Rng::new();

    // If we cannot sync the time in 5 seconds, we give up
    unsafe {
        libc::alarm(5u32);
    };

    // NTP epoch is 1900-01-01T00:00:00Z
    // Unix epoch is 1970-01-01T00:00:00Z
    let ntp_unix_offset = 2208988800u64;

    let pool_host = "pool.ntp.org:123";
    if let Ok(addresses) = std::net::ToSocketAddrs::to_socket_addrs(pool_host) {
        let addresses = addresses.collect::<Vec<_>>();
        let address = addresses[rng.next() as usize % addresses.len()];
        let sock = std::net::UdpSocket::bind("0.0.0.0:0").expect("Cannot create UDP socket");

        let mut buf = [0; 48];
        buf[0] = 35; // leap indicator = 0, version = 4, mode = 3

        sock.send_to(&buf, address).unwrap();

        if let Ok((48, _)) = sock.recv_from(&mut buf) {
            let transmit_timestamp_seconds = u32::from_be_bytes(buf[40..44].try_into().unwrap());
            let transmit_timestamp_fraction = u32::from_be_bytes(buf[44..48].try_into().unwrap());

            let millis_now = get_millis();
            let transmit_time = (transmit_timestamp_seconds as u64 - ntp_unix_offset) as f64
                + (transmit_timestamp_fraction as f64 / 2.0f64.powi(32));

            let diff = transmit_time - (millis_now as f64 / 1000.0);

            println!("Adjusting time by {} seconds", diff);

            let timeval = libc::timeval {
                tv_sec: diff.trunc() as i64,
                tv_usec: (diff.fract() * 1_000_000.0).trunc() as i64,
            };
            let res = unsafe { libc::adjtime(&timeval, std::ptr::null_mut()) };
            if res != 0 {
                eprintln!("Failed to adjust time, are you running as root? Or is the difference too large?");
            }
        } else {
            eprintln!("Failed to receive NTP response");
        }
    } else {
        eprintln!("Unable to resolve pool host: {}", pool_host);
    }
}
