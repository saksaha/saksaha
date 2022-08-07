use chacha20::ChaCha20;
// Import relevant traits
use chacha20::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};
use hex_literal::hex;

#[tokio::test(flavor = "multi_thread")]
async fn test_chacha20() {
    let key = [0x42; 32];
    let nonce = [0x24; 12];
    let plaintext = hex!("00010203 04050607 08090a0b 0c0d0e0f");
    let ciphertext = hex!("e405626e 4f1236b3 670ee428 332ea20e");

    // Key and IV must be references to the `GenericArray` type.
    // Here we use the `Into` trait to convert arrays into it.
    let mut cipher = ChaCha20::new(&key.into(), &nonce.into());

    let mut buffer = plaintext.clone();

    println!("buffer: {:?}", buffer);

    // apply keystream (encrypt)
    cipher.apply_keystream(&mut buffer);
    assert_eq!(buffer, ciphertext);

    let ciphertext = buffer.clone();

    // ChaCha ciphers support seeking
    cipher.seek(0u32);

    // decrypt ciphertext by applying keystream again
    cipher.apply_keystream(&mut buffer);
    assert_eq!(buffer, plaintext);

    // stream ciphers can be used with streaming messages
    cipher.seek(0u32);
    for chunk in buffer.chunks_mut(3) {
        cipher.apply_keystream(chunk);
    }
    assert_eq!(buffer, ciphertext);
}
