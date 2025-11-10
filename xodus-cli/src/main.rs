use xodus::HttpCallbackHandler;
use xodus::xal;
use xal::{
    Constants, Flows, XalAppParameters, XalAuthenticator,
    client_params::CLIENT_WINDOWS,
    oauth2::{RedirectUrl, Scope},
};

#[tokio::main]
async fn main() {
    let app_params = XalAppParameters {
        client_id: xodus::CLIENT_ID.into(),
        title_id: None,
        auth_scopes: vec![
            Scope::new(xal::Constants::SCOPE_XBL_SIGNIN.to_owned()),
            Scope::new(xal::Constants::SCOPE_XBL_OFFLINE_ACCESS.to_owned()),
        ],
        redirect_uri: Some(RedirectUrl::new(xodus::REDIRECT_URI.into()).unwrap()),
        client_secret: Some(xodus::CLIENT_SECRET.to_owned()),
    };
    let mut authenticator = XalAuthenticator::new(app_params, CLIENT_WINDOWS(), "RETAIL".into());
    let ts = Flows::ms_authorization_flow(
        &mut authenticator,
        HttpCallbackHandler {
            bind_host: "127.0.0.1:9001".to_owned(),
            redirect_url_base: "http://localhost:9001".to_owned(),
        },
        false,
    )
    .await
    .expect("Failed to login");

    let mut ts = Flows::xbox_live_authorization_traditional_flow(
        &mut authenticator,
        ts.live_token,
        Constants::RELYING_PARTY_XBOXLIVE.into(),
        xal::AccessTokenPrefix::D,
        false,
    )
    .await
    .expect("Failed to get xbox live data");

    ts.update_timestamp();
    ts.save_to_file("tokens.json").expect("Failed to save");
}
