use aes_gcm_siv::{
    aead::{Aead, KeyInit, OsRng},
    Aes256GcmSiv,
    Nonce, // Or `Aes128GcmSiv`
};
#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;
    dotenvy::dotenv().ok();

    let key = Aes256GcmSiv::generate_key(&mut OsRng);
    let cipher = Aes256GcmSiv::new(&key);
    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
    let ciphertext = cipher
        .encrypt(nonce, b"plaintext message".as_ref())
        .unwrap();
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).unwrap();
    assert_eq!(&plaintext, b"plaintext message");
    Ok(())
}
