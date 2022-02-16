use {
    dotenv,
    reqwest,
    tokio,
    serde_json::Result,
    crate::blockchain_status::BlockchainStatus,
    crate::blockchain_address::BlockchainAddress,
    crate::blockchain_transaction::BlockchainTransaction,
};


const HOST_ROOT: &str = "https://btcbook.nownodes.io/api/";


#[tokio::main]
pub async fn send_request(url: &str) -> String {

    let client = reqwest::Client::new();
    client
        .get(url)
        .header("api-key", dotenv::var("API_KEY").expect("Error reading env var"))
        .send()
        .await
        .expect("Failed to get response")
        .text()
        .await
        .expect("Failed to convert to JSON")
}


pub fn blockchain_status_request() -> BlockchainStatus {
    let response = send_request(&HOST_ROOT);
    serde_json::from_str(&response).expect("Failed to parse JSON")
}

pub fn blockchain_address_request(address_address: &str) -> BlockchainAddress {
    let response = send_request(&[HOST_ROOT, "v2/address/", &address_address].join(""));
    serde_json::from_str(&response).expect("Failed to parse JSON")
}

pub fn blockchain_transaction_request(transaction_id: &str) -> BlockchainTransaction {
    let response = send_request(&[HOST_ROOT, "v2/tx/", &transaction_id].join(""));
    serde_json::from_str(&response).expect("Failed to parse JSON")
}