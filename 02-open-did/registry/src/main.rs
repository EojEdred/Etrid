mod did;

use did::{DIDDocument, create_did};

fn main() {
    println!("Ëtrid OpenDID system booting...");
    let doc = create_did("Eoj Edred");
    println!("Generated DID Document: {}", serde_json::to_string_pretty(&doc).unwrap());
}
