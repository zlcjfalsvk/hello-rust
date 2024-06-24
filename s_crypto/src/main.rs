use base64::{engine::general_purpose, Engine as _};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key,
};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

fn main() {
    // println!("start asymmetric_encryption_rsa");
    // asymmetric_encryption_rsa();
    // println!("-------------------------------");
    // println!("start symmetric_encryption_chacha20");
    symmetric_encryption_chacha20().unwrap();
}

fn asymmetric_encryption_rsa() {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    // Encrypt
    let data = b"hello world";
    let enc_data = pub_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &data[..])
        .expect("failed to encrypt");
    assert_ne!(&data[..], &enc_data[..]);

    println!(
        "data: {:?}: enc_data: {:?}",
        String::from_utf8(Vec::from(data)).unwrap(),
        enc_data
    );

    // Decrypt
    let dec_data = priv_key
        .decrypt(Pkcs1v15Encrypt, &enc_data)
        .expect("failed to decrypt");
    assert_eq!(&data[..], &dec_data[..]);

    println!("dec_data: {:?}", String::from_utf8(dec_data).unwrap(),);
}

fn symmetric_encryption_chacha20() -> Result<(), chacha20poly1305::Error> {
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let key_base64 = general_purpose::STANDARD.encode(&key);
    println!("Key: {:?}", key);
    println!("Base64 Encoded Key: {}", key_base64);

    let decoded_key = general_purpose::STANDARD
        .decode(&key_base64)
        .expect("Failed to decode Base64 key");
    let key = Key::from_slice(&decoded_key);

    println!("Base64 Decoded Key: {:?}", key);

    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message

    let data = b"hello world";
    let ciphertext = cipher.encrypt(&nonce, data.as_ref())?;
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref())?;

    println!(
        "data: {:?}: enc_data: {:?}",
        String::from_utf8(Vec::from(data)).unwrap(),
        ciphertext
    );

    println!("dec_data: {:?}", String::from_utf8(plaintext).unwrap(),);

    Ok(())
}
