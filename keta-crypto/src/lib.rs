mod keys;
mod signature;

pub use signature::Signature;
pub use signature::Error as SignatureError;

pub use keys::PublicKey;
pub use keys::SecretKey;
pub use keys::Error as PublicKeyError;
pub use keys::Error as SecretKeyError;

#[derive(Clone, PartialEq, Eq)]
pub struct Keypair {
    pub public: PublicKey,
    pub secret: SecretKey,
}

impl Keypair {
    pub const ZERO: Self = Self {
        public: PublicKey::ZERO,
        secret: SecretKey::ZERO,
    };

    pub fn generate() -> Self {
        use rand::prelude::ThreadRng;
        let mut csprng: ThreadRng = rand::thread_rng();
        let kp = ed25519_dalek::Keypair::generate(&mut csprng);

        Self {
            public: PublicKey::from_ed25519_dalek(kp.public),
            secret: SecretKey::from_ed25519_dalek(kp.secret),
        }
    }

    pub fn sign(&self, message: impl AsRef<[u8]>) -> Signature {
        use ed25519_dalek::Signer;
        let signature = self.to_ed25519_dalek().sign(message.as_ref());

        Signature::from_ed25519_dalek(signature)
    }

    pub fn to_ed25519_dalek(&self) -> ed25519_dalek::Keypair {
        ed25519_dalek::Keypair {
            public: self.public.to_ed25519_dalek(),
            secret: self.secret.to_ed25519_dalek(),
        }
    }
}