use hex::ToHex;
use js_sys::Array;
use sha3::{Digest, Sha3_512};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn find_hash(prefixes: Vec<String>, prepend: String, should_find_match: bool) -> JsValue {
    let nonce = Arc::new(AtomicUsize::new(0));

    if let Some((input, index)) = prefixes.iter().enumerate().find_map(|(i, prefix)| {
        let mut hasher = Sha3_512::new();
        loop {
            let input = format!("{}{}", prepend, nonce.fetch_add(1, Ordering::Relaxed));
            hasher.update(input.as_bytes());
            let hash = hasher.finalize_reset().encode_hex::<String>();

            if should_find_match && hash.starts_with(prefix.as_str()) {
                return Some((input, i));
            }

            if !should_find_match {
                return None;
            }
        }
    }) {
        let result = Array::new();
        result.push(&JsValue::from_str(&input));
        result.push(&JsValue::from(index as u32));
        JsValue::from(result)
    } else {
        JsValue::null()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_find_hash() {
        let prefixes = vec![
            "00".to_string(),
            "11".to_string(),
            "22".to_string(),
            "33".to_string(),
        ];
        let prepend = "test".to_string();

        let result = find_hash(prefixes.clone(), prepend.clone(), true);
        assert!(!result.is_null());

        let array = js_sys::Array::from(&result);
        let input = array.get(0).as_string().unwrap();
        let index = array.get(1).as_f64().unwrap() as usize;

        let mut hasher = Sha3_512::new();
        hasher.update(input.as_bytes());
        let hash = hasher.finalize().encode_hex::<String>();
        assert!(hash.starts_with(&prefixes[index]));

        let result = find_hash(prefixes, prepend, false);
        assert!(result.is_null());
    }
}