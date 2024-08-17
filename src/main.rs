use futures::{StreamExt, TryStreamExt};
use ipfs_api::{IpfsApi, IpfsClient};
use ipfsapi::IpfsApi as CustomIpfsApi;
use serde_json::Value;
use std::fs::File;
use std::io::{Cursor, Read};
use tokio;

#[tokio::main]
async fn main() {
    // Create an IPFS client
    let client = IpfsClient::default();
    // Load the JSON file
    for i in 1..4 {
        let mut file = File::open(format!("./metadata-{}.json", i)).expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read file");
        let data = Cursor::new(contents);

        // Upload the JSON data to IPFS
        match client.add(data).await {
            Ok(res) => println!("Uploaded to IPFS with CID: {}", res.hash),
            Err(e) => eprintln!("Failed to upload to IPFS: {}", e),
        };
    }

    let data = retrieve_from_ipfs("QmUavHLz5j9oeaPiuoZ1pVHzVH1cBNiyTh1dzgjKN8H5zP")
        .await
        .unwrap();
    println!("{}", data);
}

// Function to retrieve JSON data from IPFS
async fn retrieve_from_ipfs(cid: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api = CustomIpfsApi::new("127.0.0.1", 5001);

    let bytes = api.cat(cid).unwrap();
    let data = String::from_utf8(bytes.collect()).unwrap();

    Ok(data)
}
