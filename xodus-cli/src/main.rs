use crate::webview::WebviewCallbackHandler;
use clap::{Parser, Subcommand};
mod commands;
mod webview;

use xodus::xal::TokenStore;
use xodus::xal::client_params::CLIENT_WINDOWS;
use xodus::xal::oauth2::TokenResponse;

#[derive(Subcommand)]
enum SubCommand {
    Download {
        product: String,
        #[arg(short, long)]
        market: Option<String>,
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },
    License {
        content_id: String,
        #[arg(short, long)]
        market: Option<String>,
    },
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[command(subcommand)]
    command: SubCommand,
}

#[tokio::main]
async fn main() {
    env_logger::init_from_env("XODUS_LOG");
    let client = reqwest::ClientBuilder::new()
        .user_agent(CLIENT_WINDOWS().user_agent)
        .build()
        .unwrap();
    let args = CliArgs::parse();
    log::info!("Logging in...");

    let mut ts = match TokenStore::load_from_file("tokens.json") {
        Ok(mut ts) => {
            if ts
                .updated
                .zip(ts.live_token.expires_in())
                .is_none_or(|(up, exp)| up + exp < chrono::Utc::now())
            {
                match xodus::xal::Flows::try_refresh_live_tokens_from_tokenstore(&mut ts).await {
                    Ok(mut authenticator) => {
                        xodus::auth::refresh_tokens(&mut authenticator, ts.live_token)
                            .await
                            .expect("Failed to refresh tokens")
                    }
                    Err(_) => xodus::auth::start_new_session(WebviewCallbackHandler)
                        .await
                        .expect("Failed to login"),
                }
            } else {
                ts
            }
        }
        Err(_) => xodus::auth::start_new_session(WebviewCallbackHandler)
            .await
            .expect("Failed to login"),
    };
    ts.update_timestamp();
    ts.save_to_file("tokens.json").expect("Failed to save");

    match args.command {
        SubCommand::Download {
            product,
            market,
            dry_run,
        } => commands::download::run(&client, &ts, product, market, dry_run).await,
        SubCommand::License {
            content_id: _,
            market: _,
        } => {
            unimplemented!("This codepath doesnt work yet!");
            // let res = licensing::content::get_license_content(
            //     &client,
            //     &ts,
            //     content_id,
            //     market.unwrap_or("US".into()),
            // )
            // .await
            // .expect("Failed to get license");

            // println!("{res}");
        }
    }
}
