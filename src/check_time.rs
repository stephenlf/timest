use anyhow::Result;

const NPT_ADDR: &str = "time.nist.gov:123";
// const PERMISSIBLE_ERROR_MIN: u64 = 15;
const PERMISSIBLE_ERROR_MIN: u64 = 15;

pub fn check_time() -> Result<()> {
    let sys_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let nist_time = get_time();
    let diff = diff(sys_time + 2208988800_u64, nist_time?);
    if diff <= 60 * PERMISSIBLE_ERROR_MIN {
        Ok(())
    } else {
        Err(anyhow::anyhow!("system clock is more than {PERMISSIBLE_ERROR_MIN} minute out of date"))
    }
}

pub fn prompt_err(error_msg: &str) -> Result<()> {
    println!("Whoops! Error: {error_msg}. Are you sure you want to continue? (y/n)");

    let mut input = String::with_capacity(2);
    std::io::stdin().read_line(&mut input)?;

    match input.trim() {
        "y" | "Y" | "yes" | "YES" | "Yes" => Ok(()),
        _ => Err(anyhow::anyhow!("User-initiated shutdown"))
    }
}

pub fn shutdown(conn: sqlite::Connection) -> ! {
    println!("Shutting down");
    drop(conn);
    panic!("Exiting");
}

fn get_time() -> Result<u64> {
    let response = ntp::request(NPT_ADDR);
    if response.is_err() {
        return Err(anyhow::anyhow!("failed to communicate with time server"));
    }
    let ntp_time = response.unwrap().transmit_time.sec;
    Ok(ntp_time as u64)
}

fn diff(a: u64, b: u64) -> u64 {
    if a > b {
        a - b
    } else {
        b - a
    }
}