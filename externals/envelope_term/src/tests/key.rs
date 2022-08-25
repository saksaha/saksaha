use sak_crypto::SecretKey;
use sak_test_utils::init_test_log;

#[test]
fn test_recover_secret_key_from_the_credential() {
    init_test_log();

    let secret_key: String = String::from(
        "\
        224d0898389759f29ad5c9a6472b26fff86b629388988eec457a88ce50e907a0",
    );

    let mut secret_key = secret_key.as_bytes().to_vec();

    println!("[+] secret_key: {:?}", secret_key);
    println!("[+] secret_key (len): {:?}", secret_key.len());

    let mut pad = vec![0x0u8; 192];
    secret_key.append(&mut pad);

    let tmp: &[u8] = secret_key.as_ref();

    println!("\n[!] after padding\n");
    println!("[+] secret_key (len): {:?}", tmp.len());

    let a = match SecretKey::from_bytes(secret_key) {
        Ok(a) => a,
        Err(err) => {
            println!("Error msg: {:?}", err);

            panic!();
        }
    };

    println!("res: {:?}", a);
}
