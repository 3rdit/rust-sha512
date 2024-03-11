use hex::ToHex;
use sha3::{Digest, Sha3_512};
use rust_sha512::find_hash;

fn main() {
    let prefixes = vec![
        "00".to_string(),
        "11".to_string(),
        "22".to_string(),
        "33".to_string(),
    ];
    let prepend = "some_string_to_prepend".to_string();

    let result = find_hash(prefixes, prepend, true);
    if !result.is_null() {
        let array = js_sys::Array::from(&result);
        let input = array.get(0).as_string().unwrap();
        let index = array.get(1).as_f64().unwrap() as usize;

        web_sys::console::log_1(&format!("Found input: {}", input).into());
        web_sys::console::log_1(&format!("Index in the array: {}", index).into());

        let mut hasher = Sha3_512::new();
        hasher.update(input.as_bytes());
        let hash = hasher.finalize().encode_hex::<String>();
        web_sys::console::log_1(&format!("Corresponding hash: {}", hash).into());
    } else {
        web_sys::console::log_1(&"No matching hash found.".into());
    }
}