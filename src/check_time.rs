const NPT_ADDR: &str = "time.nist.gov:123";

pub fn check_time() -> Result<(), &'static str> {
    let sys_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let nist_time = get_time();
    let diff = diff(sys_time + 2208988800_u64, nist_time);
    if diff <= 60*15 {
        Ok(())
    } else {
        Err("System clock is more than one minute out of date")
    }
}


fn get_time() -> u64 {
    let response = ntp::request(NPT_ADDR).unwrap();
    let ntp_time = response.transmit_time.sec;
    ntp_time as u64
}

fn diff(a: u64, b: u64) -> u64 {
    if a > b {
        a - b
    } else {
        b - a
    }
}