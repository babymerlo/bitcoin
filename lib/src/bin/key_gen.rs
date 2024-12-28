use std::env;

use btclib::crypto::PrivateKey;
use btclib::util::Saveable;

fn main() {
    let name = env::args().nth(1).expect("Please provide file name");
    let private_key = PrivateKey::new_key();
    let pub_key = private_key.public_key();
    let pub_key_file = name.clone() + ".pub.pem";
    let private_key_file = name + ".priv.cbor";
    private_key.save_to_file(&private_key_file).unwrap();
    pub_key.save_to_file(&pub_key_file).unwrap();
}
