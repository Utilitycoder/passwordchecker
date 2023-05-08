use reqwest;
use sha1::{Digest, Sha1};

pub fn check_password(password: &str) -> Result<bool, reqwest::Error> {
    let hash = Sha1::digest(password.as_bytes());
    let hash = format!("{:X}", hash);
    let hash = hash[..5].to_string();
    let hash = hash.to_uppercase();

    let url = format!("https://api.pwnedpasswords.com/range/{}", hash);
    let response = reqwest::blocking::get(&url)?;
    let response = response.text()?;
    let response = response.split("\r\n");
    let mut found = false;

    for line in response {
        let line = line.split(":").collect::<Vec<&str>>();
        if line[0] == hash {
            found = true;
            break;
        }
    }
    Ok(found)
}