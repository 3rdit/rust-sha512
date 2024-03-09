use crossbeam::channel;
use hex::ToHex;
use rayon::prelude::*;
use sha3::{Digest, Sha3_512};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

fn find_hash(prefixes: Arc<Vec<String>>, prepend: Arc<String>, timeout: Option<Duration>, should_find_match: bool) -> Option<(String, usize)> {
    let (sender, receiver) = channel::unbounded();
    let nonce = Arc::new(AtomicUsize::new(0));

    prefixes.par_iter().enumerate().find_any(|(i, prefix)| {
        let mut hasher = Sha3_512::new();
        loop {
            let input = format!("{}{}", prepend, nonce.fetch_add(1, Ordering::Relaxed));
            hasher.update(input.as_bytes());
            let hash = hasher.finalize_reset().encode_hex::<String>();
            if should_find_match && hash.starts_with(prefix.as_str()) {
                sender.send((input, *i)).unwrap();
                return true;
            }
            if !should_find_match {
                return false;
            }
        }
    });

    if let Some(duration) = timeout {
        receiver.recv_timeout(duration).ok()
    } else {
        receiver.recv().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_hash_single_prefix() {
        let prefixes = vec!["00".to_string()];
        let prepend = "test".to_string();

        if let Some((input, index)) = find_hash(Arc::new(prefixes.clone()), Arc::new(prepend.clone()), None, true) {
            assert_eq!(index, 0);

            let mut hasher = Sha3_512::new();
            hasher.update(input.as_bytes());
            let hash = hasher.finalize().encode_hex::<String>();
            assert!(hash.starts_with(&prefixes[index]));
        } else {
            panic!("No matching hash found");
        }
    }

    #[test]
    fn test_find_hash_multiple_prefixes() {
        let prefixes = vec![
            "00".to_string(),
            "11".to_string(),
            "22".to_string(),
            "33".to_string(),
        ];
        let prepend = "test".to_string();

        if let Some((input, index)) = find_hash(Arc::new(prefixes.clone()), Arc::new(prepend.clone()), None, true) {
            assert!(index < prefixes.len());

            let mut hasher = Sha3_512::new();
            hasher.update(input.as_bytes());
            let hash = hasher.finalize().encode_hex::<String>();
            assert!(hash.starts_with(&prefixes[index]));
        } else {
            panic!("No matching hash found");
        }
    }

    #[test]
    fn test_find_hash_timeout() {
        let prefixes = vec![
            "00".to_string(),
            "11".to_string(),
            "22".to_string(),
            "33".to_string(),
        ];
        let prepend = "test".to_string();

        let timeout = Some(Duration::from_millis(100));

        let start_time = std::time::Instant::now();
        let result = find_hash(Arc::new(prefixes), Arc::new(prepend), timeout, false);
        let elapsed_time = start_time.elapsed();

        assert!(result.is_none());
        assert!(elapsed_time >= Duration::from_millis(100));
        assert!(elapsed_time < Duration::from_millis(200));
    }
}

fn main() {
    let prefixes = vec![
        "00".to_string(),
        "11".to_string(),
        "22".to_string(),
        "33".to_string(),
    ];
    let prepend = "some_string_to_prepend".to_string();

    if let Some((input, index)) = find_hash(Arc::new(prefixes), Arc::new(prepend), None, true) {
        println!("Found input: {}", input);
        println!("Index in the array: {}", index);

        let mut hasher = Sha3_512::new();
        hasher.update(input.as_bytes());
        let hash = hasher.finalize().encode_hex::<String>();
        println!("Corresponding hash: {}", hash);
    } else {
        println!("No matching hash found.");
    }
}