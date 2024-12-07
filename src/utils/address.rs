use ring::digest::{Context, Digest, SHA256};
use ring::rand::{SecureRandom, SystemRandom};
use ring::signature::{Ed25519KeyPair, KeyPair};
use ripemd::{Ripemd160, Digest as RipemdDigest};
use bs58::{decode, encode};
use sha2::Sha256;

const BASE58_ALPHABET: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

fn create_keypair() -> Ed25519KeyPair {
    let rng = SystemRandom::new();
    let mut seed = [0u8; 32];
    rng.fill(&mut seed).unwrap();
    Ed25519KeyPair::from_seed_unchecked(&seed).unwrap()
}

/// We use same impl as bitcoin addresses
fn validate_tzkbc_address(address: &str) -> bool {
    let decoded_address = match decode(address).into_vec() {
        Ok(result) => result,
        Err(_) => return false,
    };
    // Check the length of the decoded address
    if decoded_address.len() != 25 {
        return false;
    }
    // Split the decoded address into the payload and checksum
    let (payload, checksum) = decoded_address.split_at(21);
    // Calculate the checksum of the payload
    let calculated_checksum = Sha256::digest(&Sha256::digest(payload));
    // Compare the first 4 bytes of the calculated checksum with the provided checksum
    &calculated_checksum[0..4] == checksum
}

/// We use same impl as bitcoin addresses
/// Used for testing and scripting
fn create_tzkbc_address() -> String {
    let keypair = create_keypair();
    let public_key = keypair.public_key().as_ref();

    // Perform SHA-256 hashing on the public key
    let sha256_hash = sha256(public_key);

    // Perform RIPEMD-160 hashing on the result of SHA-256
    let ripemd160_hash = Ripemd160::digest(sha256_hash.as_ref());
    // Add network byte (0x00 for mainnet)
    let mut address_bytes = vec![0x00];
    address_bytes.extend(&ripemd160_hash);

    // Perform double SHA-256 to get the checksum
    let binding = sha256(&sha256(&address_bytes).as_ref());
    let checksum = binding.as_ref();
    address_bytes.extend(&checksum[0..4]);

    // Convert to base58
    encode(String::from_utf8(address_bytes).expect("big problems")).into_string()
}

fn sha256(data: &[u8]) -> Digest {
    let mut context = Context::new(&SHA256);
    context.update(data);
    context.finish()
}
