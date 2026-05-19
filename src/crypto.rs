use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::OsRng;
pub struct Keypair {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

impl Keypair {
    pub fn generate() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        Self {
            signing_key,
            verifying_key,
        }
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }

    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.verifying_key.to_bytes()
    }

    pub fn public_key_hex(&self) -> String {
        hex::encode(self.verifying_key.to_bytes())
    }
}

impl std::fmt::Display for Keypair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Keypair")
            .field("public_key", &self.public_key_hex())
            .finish()
    }
}

pub fn transfer_message(from: u64, to: u64, amount: i64, nonce: u64) -> [u8; 32] {
    let mut buf = [0u8; 32];
    buf[0..8].copy_from_slice(&from.to_le_bytes());
    buf[8..16].copy_from_slice(&to.to_le_bytes());
    buf[16..24].copy_from_slice(&amount.to_le_bytes());
    buf[24..32].copy_from_slice(&nonce.to_le_bytes());
    buf
}

pub fn verify(
    public_key_bytes: &[u8; 32],
    message: &[u8],
    signature: &Signature,
) -> Result<(), ed25519_dalek::SignatureError> {
    let vk = VerifyingKey::from_bytes(public_key_bytes)?;
    vk.verify(message, signature)
}
