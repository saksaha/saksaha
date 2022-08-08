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

#[tokio::test(flavor = "multi_thread")]
async fn test_chacha20_two_parties() {
    let key = [0x42; 32];
    let nonce = [0x24; 12];
    let plaintext = hex!("00010203 04050607 08090a0b 0c0d0e0f");
    let ciphertext = hex!("e405626e 4f1236b3 670ee428 332ea20e");

    // Key and IV must be references to the `GenericArray` type.
    // Here we use the `Into` trait to convert arrays into it.
    let mut cipher1 = ChaCha20::new(&key.into(), &nonce.into());

    let mut cipher2 = ChaCha20::new(&key.into(), &nonce.into());

    let mut buffer1 = plaintext.clone();

    println!("\noriginal buffer: {:?}", buffer1);

    cipher1.apply_keystream(&mut buffer1);

    println!("cipher encrypts, buf1: {:?}", buffer1);

    assert_eq!(buffer1, ciphertext);

    let mut buffer2 = buffer1.clone();

    cipher2.apply_keystream(&mut buffer2);

    println!("cipher2 deciphers, buf: {:?}", buffer2);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_chacha20_two_parties_async_fail() {
    let key = [0x42; 32];
    let nonce = [0x24; 12];
    let plaintext = hex!("00010203 04050607 08090a0b 0c0d0e0f");
    let ciphertext = hex!("e405626e 4f1236b3 670ee428 332ea20e");

    // Key and IV must be references to the `GenericArray` type.
    // Here we use the `Into` trait to convert arrays into it.
    let mut cipher1 = ChaCha20::new(&key.into(), &nonce.into());

    let mut cipher2 = ChaCha20::new(&key.into(), &nonce.into());

    let mut buffer1 = plaintext.clone();

    println!("\noriginal buffer: {:?}", buffer1);

    cipher1.apply_keystream(&mut buffer1);

    let intermediate_buf1 = buffer1.clone();

    println!("cipher encrypts, buf1: {:?}", buffer1);

    cipher1.apply_keystream(&mut buffer1);

    println!("cipher encrypts, buf1: {:?}", buffer1);

    let mut buffer2 = buffer1.clone();

    cipher2.apply_keystream(&mut buffer2);

    let intermediate_buf2 = buffer2.clone();

    println!("cipher2 deciphers, buf2: {:?}", buffer2);

    cipher2.apply_keystream(&mut buffer2);

    println!("cipher2 deciphers, buf2: {:?}", buffer2);

    assert_ne!(
        intermediate_buf1, intermediate_buf2,
        "Even though each cipher has applied keystream once, the results are\
        not going to be identical",
    );
}
