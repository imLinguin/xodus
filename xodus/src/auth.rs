use std::str::from_utf8;

use async_trait::async_trait;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};

use dotenv_codegen::dotenv;
use reqwest::Url;
use xal::{AuthPromptCallback, AuthPromptData};

pub const CLIENT_ID: &str = dotenv!("CLIENT_ID");
pub const CLIENT_SECRET: &str = dotenv!("CLIENT_SECRET");
pub const REDIRECT_URI: &str = "http://localhost:9001/xodus/redirect";

pub struct HttpCallbackHandler {
    pub bind_host: String,
    pub redirect_url_base: String,
}

#[async_trait]
impl AuthPromptCallback for HttpCallbackHandler {
    async fn call(
        &self,
        cb_data: AuthPromptData,
    ) -> Result<Option<Url>, Box<dyn std::error::Error>> {
        let prompt = cb_data.prompt();
        println!("{prompt}\n");

        let listener = TcpListener::bind(&self.bind_host).await?;
        println!("HTTP Server listening, waiting for connection...");

        let (mut socket, addr) = listener.accept().await?;
        println!("Connection received from {addr:?}");

        let mut buf = [0u8; 1024];

        if socket.read(&mut buf).await? == 0 {
            return Err("Failed reading http request".into());
        }

        socket.write_all(b"HTTP/1.1 200 OK\n\r\n\rGo back to your app!").await?;

        let http_req = from_utf8(&buf)?;

        let path = http_req.split(' ').nth(1).unwrap();

        Ok(Some(Url::parse(&format!(
            "{}{}",
            self.redirect_url_base, path
        ))?))
    }
}