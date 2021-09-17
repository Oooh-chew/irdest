use crate::{
    crypto::CipherText,
    error::{Error, Result},
    io::{Decode, Encode},
};
use serde::{de::DeserializeOwned, Serialize};
use sodiumoxide::crypto::box_::{self, Nonce, PublicKey, SecretKey};

/// A wrapper around an NaCl public key
pub struct PubKey {
    inner: PublicKey,
}

impl PubKey {
    pub(crate) fn seal(&self, data: &[u8], auth: &SecKey) -> CipherText {
        let non = box_::gen_nonce();
        let data = box_::seal(&data, &non, &self.inner, &auth.inner);
        let nonce = non.0.iter().cloned().collect();
        CipherText { nonce, data }
    }
}

/// A wrapper around an NaCl secret key
pub struct SecKey {
    inner: SecretKey,
}

impl SecKey {
    pub(crate) fn open<T>(&self, data: &CipherText, auth: &PubKey) -> Result<T>
    where
        T: Decode<T> + Serialize,
    {
        let CipherText {
            ref nonce,
            ref data,
        } = data;

        let nonce =
            Nonce::from_slice(&nonce.as_slice()).ok_or(Error::internal("Failed to read nonce!"))?;

        let clear = box_::open(data.as_slice(), &nonce, &auth.inner, &self.inner)
            .map_err(|_| Error::internal("Failed to decrypt data"))?;

        Ok(T::decode(&clear)?)
    }
}
