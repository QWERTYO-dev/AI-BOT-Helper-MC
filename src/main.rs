use azalea::prelude::*;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Keep-alive server for Render/Cloud hosts
    tokio::spawn(async move {
        let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
        println!("Keep-alive server running on port 8080");
        loop {
            let _ = listener.accept().await;
        }
    });

    println!("Starting Minecraft bot...");
    let account = Account::offline("CompanionBot");
    
    // REPLACE 'YOUR_SERVER_IP' WITH YOUR ACTUAL SERVER IP/DOMAIN
    ClientBuilder::new()
        .start(account, "DZ_FUN.aternos.me:24203")
        .await;
}
