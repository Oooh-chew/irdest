//! Provides more convenient crypto wrappers
#![allow(unused)]

pub(crate) mod pkcry;

pub(crate) mod aes;

#[deprecated]
pub(crate) mod asym;

pub(crate) mod bs;

mod hidden;
pub(crate) use hidden::Hid;

mod map;
pub(crate) use map::EncryptedMap;

use crate::{
    error::{Error, Result},
    utils::Id,
    wire::Encoder,
};
use async_std::sync::Arc;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{fmt::Debug, marker::PhantomData};

/// An encrypted piece of data
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub(crate) struct CipherText {
    /// Number only used once
    nonce: Vec<u8>,
    /// Data buffer
    data: Vec<u8>,
}

/// A trait that encrypts data on an associated key
#[deprecated]
pub(crate) trait Encrypter<T>
where
    T: Encoder<T>,
{
    fn seal(&self, data: &T) -> Result<CipherText>;
    fn open(&self, data: &CipherText) -> Result<T>;
}

/// A type that can provide an out-of-band key
///
/// Sometimes a type that is stored inside the `Encrypted` can bring
/// it's own key, to avoid having to have a second control-structure
/// for the keys.
#[deprecated]
pub(crate) trait DetachedKey<K> {
    fn key(&self) -> Option<Arc<K>> {
        None
    }
}

// Ids are special and should just impl this
impl<K> DetachedKey<K> for Id {}

/// A generic wrapper around the unlock state of data
#[derive(Clone, Debug, Serialize, Deserialize)]
#[deprecated]
pub(crate) enum Encrypted<T, K>
where
    T: Debug + Clone + Encoder<T> + DetachedKey<K>,
    K: Encrypter<T>,
{
    /// An in-use data variant
    #[serde(skip_serializing)]
    #[serde(bound(deserialize = "T: DeserializeOwned"))]
    Open(T),
    /// An encrypted value
    Closed(CipherText),

    /// Purely here to make rustc happy about the generic bounds
    #[doc(hidden)]
    #[serde(skip)]
    Never(Option<PhantomData<K>>),
}

impl<T, K> Encrypted<T, K>
where
    T: Debug + Clone + Encoder<T> + DetachedKey<K>,
    K: Encrypter<T>,
{
    pub(crate) fn new(init: T) -> Self {
        Self::Open(init)
    }

    /// Check if the value is encrypted
    pub(crate) fn encrypted(&self) -> bool {
        match self {
            Self::Closed(_) => true,
            _ => false,
        }
    }

    /// Attempt to deref the entry
    pub(crate) fn deref<'s>(&'s self) -> Result<&'s T> {
        match self {
            Self::Open(ref t) => Ok(t),
            _ => Err(Error::LockedState {
                msg: "Encrypted::Closed(_) can't be derefed".into(),
            }),
        }
    }

    /// Swap the underlying data in place
    pub(crate) fn swap(&mut self, new: &mut T) {
        match self {
            Self::Open(ref mut t) => std::mem::swap(t, new),
            _ => {}
        }
    }

    /// Attempt to deref the entry
    pub(crate) fn deref_mut<'s>(&'s mut self) -> Result<&'s mut T> {
        match self {
            Self::Open(ref mut t) => Ok(t),
            _ => Err(Error::LockedState {
                msg: "Encrypted::Closed(_) can't be derefed".into(),
            }),
        }
    }

    /// Call to the inner unlocked `key()` if the entry is open
    pub(crate) fn key(&self) -> Option<Arc<K>> {
        match self {
            Self::Open(t) => t.key(),
            _ => None,
        }
    }

    /// Perform the open operation in place with a key
    pub(crate) fn open(&mut self, key: &K) -> Result<()> {
        match self {
            Self::Open(_) => Err(Error::InternalError {
                msg: "tried to open ::Open(_) variant".into(),
            }),
            Self::Closed(enc) => {
                *self = Self::Open(key.open(enc)?);
                Ok(())
            }
            _ => unreachable!(),
        }
    }

    /// Perform the close operation in place with a key
    pub(crate) fn close(&mut self, key: Arc<K>) -> Result<()> {
        match self {
            Self::Closed(_) => Err(Error::InternalError {
                msg: "tried to close ::Closed(_) variant".into(),
            }),
            Self::Open(data) => {
                let key = data.key().unwrap_or(key);
                *self = Self::Closed(key.seal(data)?);
                Ok(())
            }
            _ => unreachable!(),
        }
    }

    /// Performs the close operation, purely with an detached key
    ///
    /// This function can panic and shouldn't be used unless _really_
    /// neccessary.
    pub(crate) fn close_detached(&mut self) -> Result<()> {
        match self {
            Self::Closed(_) => Err(Error::InternalError {
                msg: "tried to close ::Closed(_) variant".into(),
            }),
            Self::Open(data) => {
                let key = data.key().unwrap();
                *self = Self::Closed(key.seal(data)?);
                Ok(())
            }
            _ => unreachable!(),
        }
    }

    /// Consume the `Encrypted<T>` type into the inner value
    ///
    /// Pancis if the value is encrypted
    #[cfg(test)]
    pub(crate) fn consume(self) -> T {
        match self {
            Self::Open(data) => data,
            _ => panic!("Couldn't consume encrypted value!"),
        }
    }
}
