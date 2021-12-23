use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Reqwest(reqwest::Error);
        ParseIntError(std::num::ParseIntError);// tranform to RandomResponseError by chain_err
    }
    errors{
        RandomResponseError(t:String)
    }
}

fn parse_response(response: reqwest::blocking::Response) -> Result<u32> {
    let mut body = response.text()?;
    body.pop();
    //let body = String::from("dd");
    body.parse::<u32>()
        .chain_err(|| ErrorKind::RandomResponseError(body))
}

pub fn run() -> Result<()> {
    let url = "https://www.random.org/integers/?num=1&min=0&max=10&col=1&base=10&format=plain"
        .to_string();
    let response = reqwest::blocking::get(&url)?;
    let random_value: u32 = parse_response(response)?;
    println!("Random value between 0 and 10: {}", random_value);
    Ok(())
}
