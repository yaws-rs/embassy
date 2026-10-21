//! generate the correct form of certs & keys to be used in the server

const CA: &'static str = "../../test_certs/ca.prime256v1.crt";
const CERT: &'static str = "../../test_certs/rustcryp.to.prime256v1.ca_signed.crt";
const KEY: &'static str = "../../test_certs/rustcryp.to.prime256v1.pem";

const CA_OUT: &'static str = "../../test_certs/ca_data.bin";
const CERT_OUT: &'static str = "../../test_certs/cert_data.bin";
const KEY_OUT: &'static str = "../../test_certs/key_data.bin";

use core::convert::AsRef;

fn main() {

    println!("Pre-generatign the correct translated form of CA, CERT & KEY");
    let ca_vec = load_pem_vec(CA);
    let cert_vec = load_pem_vec(CERT);
    let key_vec = load_pem_vec(KEY);
    
    let (_cert_type_label, cert_data) = pem_rfc7468::decode_vec(&cert_vec).unwrap();
    let (_key_type_label, key_data_der) = pem_rfc7468::decode_vec(&key_vec).unwrap();
    use sec1::EcPrivateKey;
    let key_info = EcPrivateKey::try_from(key_data_der.as_ref()).unwrap();
    let key_data = key_info.private_key.to_vec();
    let (_ca_type_label, ca_data) = pem_rfc7468::decode_vec(&ca_vec).unwrap();

    println!("Writing CA out to {CA_OUT}");
    save_bin(CA_OUT, ca_data);
    
    println!("Writing CERT out to {CERT_OUT}");
    save_bin(CERT_OUT, cert_data);

    println!("Writing KEY out to {KEY_OUT}");
    save_bin(KEY_OUT, key_data);    
}

fn save_bin(path: &str, data: Vec<u8>) {
    use std::io::Write;
    let mut f = std::fs::File::create_new(path).unwrap();
    f.write(&data).unwrap();
    f.flush();
}

fn load_pem_vec(path: &str) -> Vec<u8> {
    println!("Loading {path}");
    use std::io::Read;
    let mut f = std::fs::File::open(path).unwrap();
    let mut data: Vec<u8> = vec![];
    f.read_to_end(&mut data).unwrap();
    data
}

