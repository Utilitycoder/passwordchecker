//! This crate provides a function to check if a password has been previously exposed in data breaches
//! by sending a request to the "Pwned Passwords" API provided by Cloudflare.
//!
//! # Example
//!
//! ```
//! use passwordchecker::check_passwords::check_password;
//!
//! assert_eq!(check_password("admin").unwrap(), true);
//! assert_eq!(check_password("correcthorsebatterystaple").unwrap(), false);
//! ```

use reqwest;
use sha1::{Digest, Sha1};

pub fn check_password(password: &str) -> Result<bool, reqwest::Error> {
    let hash = format!("{:X}", sha1::Sha1::digest(password.as_bytes()));
    let prefix = &hash[..5].to_uppercase();

    println!("Hash: {}", prefix);

    let url = format!("https://api.pwnedpasswords.com/range/{}", prefix);
    let response = reqwest::blocking::get(url)?.text()?;
    
    let found = response.lines().any(|line| line.starts_with(prefix));

    Ok(found)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_password() {
        let result = check_password("password");
        assert_eq!(result.unwrap(), false);
    }
}