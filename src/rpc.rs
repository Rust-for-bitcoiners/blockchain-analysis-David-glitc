use bitcoincore_rpc::{Auth, Client};
use lazy_static::lazy_static;
use dotenv::dotenv;
lazy_static! {
    pub static ref CLIENT: Client = {
        let rpc_url = std::env::var("RPC_URL").expect("BITCOIN_RPC_URL must be set");
        let rpc_user = std::env::var("RPC_USER").expect("BITCOIN_RPC_USER must be set");
        let rpc_password = std::env::var("RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD must be set");

        Client::new(
            &rpc_url,
            Auth::UserPass(rpc_user, rpc_password),
        ).expect("Failed to create client")
    };
}