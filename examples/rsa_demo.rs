use std::io;
use std::io::Read;
use rand::rngs::OsRng;
use rsa::pkcs8::{ToPrivateKey, ToPublicKey};
use rsa::RsaPrivateKey;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("开始生成...");

    let bits = 1024;
    let mut rng = OsRng;

    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");

    //写入PEM格式(PKCS#8)的私钥/公钥到文件
    private_key.write_pkcs8_pem_file("private_key.pem").expect("failed to write private key");
    private_key.write_public_key_pem_file("public_key.pem").expect("failed to write public key to file");

    println!("生成完成，按回车键关闭当前窗口");
    let mut stdin = io::stdin();
    let mut buf = [1u8; 1];
    let _ = stdin.read(&mut buf).map(|_| {});

    Ok(())
}
