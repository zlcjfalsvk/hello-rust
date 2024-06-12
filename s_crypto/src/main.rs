use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

fn main() {
    asymmetric_encryption();
    symmetric_encryption();
}

fn asymmetric_encryption() {
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

    println!(
        "data: {:?}: dec_data: {:?}",
        String::from_utf8(Vec::from(data)).unwrap(),
        String::from_utf8(dec_data).unwrap(),
    );
}

// TODO
fn symmetric_encryption() {}
