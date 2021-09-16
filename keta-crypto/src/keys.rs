use crate::Signature;
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("hex error: {0}")]
    FromHexError(#[from] hex::FromHexError),

    #[error("invalid size: {received}, expected: {expected}")]
    InvalidSize {
        expected: &'static usize,
        received: usize,
    },
}

macro_rules! impl_key {
    ($ident:ident, $size:literal) => {
        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $ident([u8; $size]);

        impl $ident {
            pub const ZERO: Self = Self([0; $size]);

            pub fn as_bytes(&self) -> &[u8; $size] {
                &self.0
            }

            pub(crate) fn to_ed25519_dalek(&self) -> ed25519_dalek::$ident {
                ed25519_dalek::$ident::from_bytes(self.as_bytes()).unwrap()
            }

            pub(crate) fn from_ed25519_dalek(val: ed25519_dalek::$ident) -> Self {
                Self(val.to_bytes())
            }
        }

        impl std::convert::TryFrom<&[u8]> for $ident {
            type Error = Error;

            fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
                let bytes_length = bytes.len();
                let bytes: [u8; $size] = bytes.try_into().map_err(|_| Error::InvalidSize {
                    expected: &$size,
                    received: bytes_length,
                })?;
                Ok(Self(bytes))
            }
        }

        impl std::str::FromStr for $ident {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let bytes = hex::decode(s)?;
                Self::try_from(bytes.as_slice())
            }
        }

        impl std::fmt::Display for $ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(hex::encode(self.0).as_str())
            }
        }

        impl std::fmt::Debug for $ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(hex::encode(self.0).as_str())
            }
        }

        impl serde::ser::Serialize for $ident {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(self.to_string().as_str())
            }
        }

        impl<'de> serde::de::Deserialize<'de> for $ident {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct Visitor;
                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = $ident;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("hex encoded key")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        <$ident as std::str::FromStr>::from_str(v).map_err(serde::de::Error::custom)
                    }
                }
                deserializer.deserialize_str(Visitor)
            }
        }

        impl AsRef<[u8]> for $ident {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }

        #[cfg(feature = "sled")]
        impl std::convert::TryFrom<sled::IVec> for $ident {
            type Error = Error;

            fn try_from(value: sled::IVec) -> Result<Self, Self::Error> {
                Self::try_from(value.as_ref())
            }
        }
    };
}

impl_key!(PublicKey, 32);
impl_key!(SecretKey, 32);

impl PublicKey {
    pub fn verify(
        &self,
        message: impl AsRef<[u8]>,
        signature: Signature,
    ) -> Result<(), ed25519_dalek::SignatureError> {
        use ed25519_dalek::Verifier;
        self.to_ed25519_dalek()
            .verify(message.as_ref(), &signature.into_ed25519_dalek())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Keypair, PublicKey, SecretKey};
    use std::str::FromStr;
    const MESSAGE: &[u8] = b"abcdefg";

    #[test]
    fn sign_verify() {
        let kp = Keypair::generate();
        let signature = kp.sign(MESSAGE);
        kp.public.verify(MESSAGE, signature).unwrap();
    }

    #[test]
    fn sign_verify_invalid_message() {
        let kp = Keypair::generate();
        let signature = kp.sign(MESSAGE);
        kp.public
            .verify(MESSAGE.iter().rev().cloned().collect::<Vec<_>>(), signature)
            .unwrap_err();
    }

    #[test]
    fn sign_verify_invalid_signer() {
        let kp = Keypair::generate();
        let kp_invalid = Keypair::generate();
        let signature = kp_invalid.sign(MESSAGE);
        kp.public.verify(MESSAGE, signature).unwrap_err();
    }

    #[test]
    fn sign_verify_invalid_skey() {
        let kp = {
            let a = Keypair::generate();
            let b = Keypair::generate();
            Keypair {
                public: a.public,
                secret: b.secret,
            }
        };
        let signature = kp.sign(MESSAGE);
        kp.public.verify(MESSAGE, signature).unwrap_err();
    }

    #[test]
    fn from_hex() {
        const PKEY: &str = "4a51e55ca2ebd01141515b6a86f0d3dd3a6b3e26a99eb733f6e3483fd92f219d";
        const SKEY: &str = "d4138dd1d994bf0e6d0244422470697e7507b6af7124f6f9da14704720ee7d86";

        let pkey = PublicKey::from_str(PKEY).unwrap();
        let skey = SecretKey::from_str(SKEY).unwrap();

        let kp = Keypair {
            public: pkey,
            secret: skey,
        };
        let _ = kp.to_ed25519_dalek(); // make sure it doesn't panic
    }

    #[test]
    fn from_hex_invalid_length() {
        // notice additional characters at the end
        const PKEY: &str = "4a51e55ca2ebd01141515b6a86f0d3dd3a6b3e26a99eb733f6e3483fd92f219dab";
        const SKEY: &str = "d4138dd1d994bf0e6d0244422470697e7507b6af7124f6f9da14704720ee7d86a";

        PublicKey::from_str(PKEY).unwrap_err();
        SecretKey::from_str(SKEY).unwrap_err();
    }
}
