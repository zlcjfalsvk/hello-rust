use base64::{engine::general_purpose, Engine as _};
use chacha20poly1305::{
    aead::{Aead as c_Aead, AeadCore as c_AeadCore, KeyInit as c_KeyInit, OsRng as c_OsRng},
    ChaCha20Poly1305, Key as c_Key,
};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

use aes_gcm::{
    aead::{Aead as g_Aead, AeadCore as g_AeadCore, KeyInit as g_KeyInit, OsRng as g_OsRng},
    Aes256Gcm,
};

fn main() {
    println!("start asymmetric_encryption_rsa");
    asymmetric_encryption_rsa();
    println!("-------------------------------");
    println!("start symmetric_encryption_chacha20");
    symmetric_encryption_chacha20().unwrap();
    symmetric_encryption_aes_gcm().unwrap();
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

fn symmetric_encryption_aes_gcm() -> Result<(), aes_gcm::Error> {
    // 1) The encryption key can be generated randomly:
    let key = Aes256Gcm::generate_key(g_OsRng);

    // 2) Transformed from a byte array:
    // let key: &[u8; 32] = &[42; 32];
    // let key: &g_Key<Aes256Gcm> = key.into();

    // 3) Note that you can get byte array from slice using the `TryInto` trait:
    // let key: &[u8] = &[42; 32];
    // let key: [u8; 32] = key.try_into()?;

    // Alternatively, the key can be transformed directly from a byte slice
    // (panicks on length mismatch):
    // let key = g_Key::<Aes256Gcm>::from_slice(key);
    let data = b"hello world";

    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut g_OsRng); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(&nonce, data.as_ref())?;
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref())?;

    println!("Encryption Data: {:?}", ciphertext);
    println!(
        "Decryption Data: {:?}",
        String::from_utf8(Vec::from(plaintext)).unwrap()
    );

    Ok(())
}

fn symmetric_encryption_chacha20() -> Result<(), chacha20poly1305::Error> {
    let key = ChaCha20Poly1305::generate_key(&mut c_OsRng);
    let key_base64 = general_purpose::STANDARD.encode(&key);
    println!("Key: {:?}", key);
    println!("Base64 Encoded Key: {}", key_base64);

    let decoded_key = general_purpose::STANDARD
        .decode(&key_base64)
        .expect("Failed to decode Base64 key");
    let key = c_Key::from_slice(&decoded_key);

    println!("Base64 Decoded Key: {:?}", key);

    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut c_OsRng); // 96-bits; unique per message

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
