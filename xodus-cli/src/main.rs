use crate::webview::WebviewCallbackHandler;

mod webview;

#[tokio::main]
async fn main() {
    println!("Logging in...");
    let mut ts = match xodus::xal::Flows::try_refresh_live_tokens_from_file("tokens.json").await {
        Ok((mut authenticator, ts)) => {
            xodus::auth::refresh_tokens(&mut authenticator, ts.live_token)
                .await
                .expect("Failed to refresh tokens")
        }
        Err(err) => {
            eprintln!("{err:?}");
            xodus::auth::start_new_session(WebviewCallbackHandler)
                .await
                .expect("Failed to login")
        }
    };
    ts.update_timestamp();
    ts.save_to_file("tokens.json").expect("Failed to save");

    let gamertag = ts
        .authorization_token
        .as_ref()
        .unwrap()
        .display_claims
        .as_ref()
        .unwrap()
        .xui
        .first()
        .unwrap()
        .get("gtg")
        .unwrap();

    println!("Logged in: {gamertag}")
}
