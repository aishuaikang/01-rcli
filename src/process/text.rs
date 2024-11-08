use crate::{cli::text::TextSignFormat, process::gen_pass::process_gen_pass, utils::get_reader};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng as ChaCha20Poly1305AeadOsRng},
    ChaCha20Poly1305, Nonce,
};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::{fs, io::Read, path::Path};

pub fn process_sign(input: &str, key: &str, format: TextSignFormat) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;
    let signed = match format {
        TextSignFormat::Blake3 => Blake3::load(key)?.sign(&mut reader)?,
        TextSignFormat::Ed25519 => Ed25519Signer::load(key)?.sign(&mut reader)?,
        TextSignFormat::ChaCha20Poly1305 => anyhow::bail!("Unsupported format"),
    };
    let signed = URL_SAFE_NO_PAD.encode(&signed);
    Ok(signed)
}

pub fn process_verify(
    input: &str,
    key: &str,
    sig: &str,
    format: TextSignFormat,
) -> anyhow::Result<bool> {
    let mut reader = get_reader(input)?;
    let sig = URL_SAFE_NO_PAD.decode(sig.as_bytes())?;
    let verified = match format {
        TextSignFormat::Blake3 => Blake3::load(key)?.verify(&mut reader, &sig)?,
        TextSignFormat::Ed25519 => Ed25519Verifier::load(key)?.verify(&mut reader, &sig)?,
        TextSignFormat::ChaCha20Poly1305 => anyhow::bail!("Unsupported format"),
    };
    Ok(verified)
}

pub fn process_gen_key(format: TextSignFormat) -> anyhow::Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
        TextSignFormat::ChaCha20Poly1305 => ChaCha20Poly1305Aead::generate(),
    }
}

pub fn process_encrypt(input: &str, key: &str) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;
    let encrypted = ChaCha20Poly1305Aead::load(key)?.encrypt(&mut reader)?;
    Ok(encrypted)
}

pub fn process_decrypt(input: &str, key: &str) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;
    let decrypted = ChaCha20Poly1305Aead::load(key)?.decrypt(&mut reader)?;
    Ok(decrypted)
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self>
    where
        Self: Sized;
}

pub trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>>;
}

pub trait TextVerify {
    fn verify(&self, reader: impl Read, sig: &[u8]) -> anyhow::Result<bool>;
}

pub trait KeyGenerator {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>>;
}

pub trait Encryptor {
    fn encrypt(&self, reader: &mut dyn Read) -> anyhow::Result<String>;
}

pub trait Decryptor {
    fn decrypt(&self, reader: &mut dyn Read) -> anyhow::Result<String>;
}

pub struct Blake3 {
    key: [u8; 32],
}

impl Blake3 {
    fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        Ok(Self::new(key))
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let key = process_gen_pass(32, true, true, true, true)?;
        let key = key.into_bytes();
        Ok(vec![key])
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signed = blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec();
        Ok(signed)
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        Ok(hash == sig)
    }
}

pub struct Ed25519Signer {
    key: SigningKey,
}

impl Ed25519Signer {
    fn new(key: SigningKey) -> Self {
        Self { key }
    }
    fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);
        Ok(Self::new(key))
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        let pk = signing_key.verifying_key().to_bytes().to_vec();
        let signing_key = signing_key.to_bytes().to_vec();
        Ok(vec![signing_key, pk])
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

impl Ed25519Verifier {
    fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        Ok(Self::new(key))
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature: Signature = Signature::from_bytes(sig.try_into()?);
        let sig = self.key.verify(&buf, &signature).is_ok();
        Ok(sig)
    }
}

pub struct ChaCha20Poly1305Aead {
    key: [u8; 32],
    nonce: Nonce,
}

impl ChaCha20Poly1305Aead {
    fn new(key: [u8; 32], nonce: Nonce) -> Self {
        Self { key, nonce }
    }

    fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        // let encrypted = URL_SAFE_NO_PAD.decode(&key)?;
        let (key, nonce) = key.split_at(32);
        let nonce = Nonce::from_slice(nonce);
        Ok(Self::new(key.try_into()?, nonce.to_owned()))
    }
}

impl KeyLoader for ChaCha20Poly1305Aead {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerator for ChaCha20Poly1305Aead {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let key = ChaCha20Poly1305::generate_key(&mut ChaCha20Poly1305AeadOsRng);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut ChaCha20Poly1305AeadOsRng); // 96-bits; unique per message
        let mut encrypted = key.to_vec();
        encrypted.extend_from_slice(&nonce);

        Ok(vec![encrypted])
    }
}

impl Encryptor for ChaCha20Poly1305Aead {
    fn encrypt(&self, reader: &mut dyn Read) -> anyhow::Result<String> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let cipher = ChaCha20Poly1305::new_from_slice(&self.key)?;
        let ciphertext = cipher.encrypt(&self.nonce, buf.as_ref()).unwrap();

        let ciphertext = URL_SAFE_NO_PAD.encode(&ciphertext);

        Ok(ciphertext)
    }
}

impl Decryptor for ChaCha20Poly1305Aead {
    fn decrypt(&self, reader: &mut dyn Read) -> anyhow::Result<String> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let buf = buf.trim_ascii();
        let buf = URL_SAFE_NO_PAD.decode(buf)?;

        let cipher = ChaCha20Poly1305::new_from_slice(&self.key)?;
        let plaintext = cipher
            .decrypt(&self.nonce, buf.as_ref())
            .map_err(|e| anyhow::anyhow!("解密密文失败: {:?}", e))?;

        Ok(String::from_utf8(plaintext).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Cursor;
    #[test]
    fn test_blake3_sign_verify() -> anyhow::Result<()> {
        let blake3 = Blake3::load("fixtrues/blake3")?;
        let data = b"hello world";
        let signed = blake3.sign(&mut Cursor::new(data))?;

        assert!(blake3.verify(&mut Cursor::new(data), &signed)?);
        Ok(())
    }

    #[test]
    fn test_ed25519_sign_verify() -> anyhow::Result<()> {
        let sk = Ed25519Signer::load("fixtrues/ed25519.sk")?;
        let pk = Ed25519Verifier::load("fixtrues/ed25519.pk")?;
        let data = b"hello world";

        let signed = sk.sign(&mut Cursor::new(data))?;
        assert!(pk.verify(&mut Cursor::new(data), &signed)?);
        Ok(())
    }

    #[test]
    fn test_cha_cha20_poly1305_aead() -> anyhow::Result<()> {
        let encrypted = ChaCha20Poly1305Aead::load("fixtrues/chacha20poly1305.key")?;
        let data = b"hello world";
        let c = encrypted.encrypt(&mut Cursor::new(data))?;
        let decrypted = encrypted.decrypt(&mut Cursor::new(c.as_bytes()))?;
        assert_eq!(data, decrypted.as_bytes());

        Ok(())
    }
}
