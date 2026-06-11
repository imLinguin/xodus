use std::collections::HashMap;

use xodus::models::secrets::{Token, TokenStore, User};

pub fn save_user(user: User) {
    let entry = xodus::secrets::get_entry("user-DA").unwrap();
    let user_str = serde_json::to_string(&user).unwrap();
    entry.set_secret(user_str.as_bytes()).unwrap();
}

pub fn get_user() -> Result<xodus::models::secrets::User, Box<dyn std::error::Error>> {
    let device_entry = xodus::secrets::get_entry("user-DA")?;
    let secret = device_entry.get_secret()?;
    let t = serde_json::from_slice::<xodus::models::secrets::User>(secret.as_slice())?;
    Ok(t)
}

pub fn save_token(address: String, token: Token) {
    let entry = xodus::secrets::get_entry("user-tokens").unwrap();
    let passwd = entry.get_password().unwrap_or_default();

    let mut tokens = if !passwd.is_empty() {
        let tokens = serde_json::from_str::<TokenStore>(&passwd).unwrap();
        tokens.tokens
    } else {
        HashMap::new()
    };
    tokens.insert(address, token);
    let tokens = TokenStore { tokens };
    let tokens_str = serde_json::to_string(&tokens).unwrap();
    entry.set_password(&tokens_str).unwrap();
}

pub fn get_token(address: String) -> Option<Token> {
    let entry = xodus::secrets::get_entry("user-tokens").unwrap();
    let passwd = entry.get_password().unwrap_or_default();

    let tokens = if !passwd.is_empty() {
        let tokens = serde_json::from_str::<TokenStore>(&passwd).unwrap();
        tokens.tokens
    } else {
        HashMap::new()
    };
    tokens.get(&address).cloned()
}
