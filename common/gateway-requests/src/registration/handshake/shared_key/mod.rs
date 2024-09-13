// Copyright 2024 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

use crate::registration::handshake::LegacySharedKeys;
use nym_crypto::generic_array::{typenum::Unsigned, GenericArray};
use nym_crypto::symmetric::aead::{self, nonce_size, AeadError, AeadKey, KeySizeUser, Nonce};
use nym_crypto::symmetric::stream_cipher::{iv_size, IV};
use nym_pemstore::traits::PemStorableKey;
use nym_sphinx::params::{GatewayEncryptionAlgorithm, LegacyGatewayEncryptionAlgorithm};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

pub mod legacy;

pub type SharedKeySize = <GatewayEncryptionAlgorithm as KeySizeUser>::KeySize;

#[derive(Debug, PartialEq)]
pub enum SharedGatewayKey {
    Current(SharedSymmetricKey),
    Legacy(LegacySharedKeys),
}

#[derive(Debug, Error)]
pub enum SharedKeyUsageError {
    #[error("the request is too short")]
    TooShortRequest,

    #[error("provided MAC is invalid")]
    InvalidMac,

    #[error("the provided nonce (or legacy IV) did not have the expected length")]
    MalformedNonce,

    #[error("did not provide a valid nonce for aead encryption")]
    MissingAeadNonce,

    #[error("failed to either encrypt or decrypt provided message")]
    AeadFailure(#[from] AeadError),
}

impl SharedGatewayKey {
    fn aead_nonce(
        raw: Option<&[u8]>,
    ) -> Result<Nonce<GatewayEncryptionAlgorithm>, SharedKeyUsageError> {
        let Some(raw) = raw else {
            return Err(SharedKeyUsageError::MissingAeadNonce);
        };
        if raw.len() != nonce_size::<GatewayEncryptionAlgorithm>() {
            return Err(SharedKeyUsageError::MalformedNonce);
        }
        Ok(Nonce::<GatewayEncryptionAlgorithm>::clone_from_slice(raw))
    }

    fn cipher_iv(
        raw: Option<&[u8]>,
    ) -> Result<Option<&IV<LegacyGatewayEncryptionAlgorithm>>, SharedKeyUsageError> {
        let Some(raw) = raw else { return Ok(None) };
        let iv = if raw.is_empty() {
            None
        } else {
            if raw.len() != iv_size::<LegacyGatewayEncryptionAlgorithm>() {
                return Err(SharedKeyUsageError::MalformedNonce);
            }
            Some(IV::<LegacyGatewayEncryptionAlgorithm>::from_slice(raw))
        };
        Ok(iv)
    }

    pub fn encrypt(
        &self,
        plaintext: &[u8],
        // the best common denominator for converting into 'IV' and 'Nonce' types
        raw_nonce: Option<&[u8]>,
    ) -> Result<Vec<u8>, SharedKeyUsageError> {
        match self {
            SharedGatewayKey::Current(aes_gcm_siv) => {
                let nonce = Self::aead_nonce(raw_nonce)?;
                aes_gcm_siv.encrypt(plaintext, &nonce)
            }
            SharedGatewayKey::Legacy(aes_ctr) => {
                let iv = Self::cipher_iv(raw_nonce)?;
                Ok(aes_ctr.encrypt_and_tag(plaintext, iv))
            }
        }
    }

    pub fn decrypt(
        &self,
        ciphertext: &[u8],
        // the best common denominator for converting into 'IV' and 'Nonce' types
        raw_nonce: Option<&[u8]>,
    ) -> Result<Vec<u8>, SharedKeyUsageError> {
        match self {
            SharedGatewayKey::Current(aes_gcm_siv) => {
                let nonce = Self::aead_nonce(raw_nonce)?;
                aes_gcm_siv.decrypt(ciphertext, &nonce)
            }
            SharedGatewayKey::Legacy(aes_ctr) => {
                let iv = Self::cipher_iv(raw_nonce)?;
                aes_ctr.decrypt_tagged(ciphertext, iv)
            }
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct SharedSymmetricKey(AeadKey<GatewayEncryptionAlgorithm>);

type KeySize = <GatewayEncryptionAlgorithm as KeySizeUser>::KeySize;

#[derive(Debug, Clone, Copy, Error)]
pub enum SharedKeyConversionError {
    #[error("the string representation of the shared key was malformed: {0}")]
    DecodeError(#[from] bs58::decode::Error),
    #[error(
        "the received shared keys had invalid size. Got: {received}, but expected: {expected}"
    )]
    InvalidSharedKeysSize { received: usize, expected: usize },
}

impl SharedSymmetricKey {
    pub fn try_from_bytes(bytes: &[u8]) -> Result<Self, SharedKeyConversionError> {
        if bytes.len() != KeySize::to_usize() {
            return Err(SharedKeyConversionError::InvalidSharedKeysSize {
                received: bytes.len(),
                expected: KeySize::to_usize(),
            });
        }

        Ok(SharedSymmetricKey(GenericArray::clone_from_slice(bytes)))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.iter().copied().collect()
    }

    pub fn try_from_base58_string<S: Into<String>>(
        val: S,
    ) -> Result<Self, SharedKeyConversionError> {
        let bs58_str = Zeroizing::new(val.into());
        let decoded = Zeroizing::new(bs58::decode(bs58_str).into_vec()?);
        Self::try_from_bytes(&decoded)
    }

    pub fn to_base58_string(&self) -> String {
        let bytes = Zeroizing::new(self.to_bytes());
        bs58::encode(bytes).into_string()
    }

    pub fn encrypt(
        &self,
        plaintext: &[u8],
        nonce: &Nonce<GatewayEncryptionAlgorithm>,
    ) -> Result<Vec<u8>, SharedKeyUsageError> {
        aead::encrypt::<GatewayEncryptionAlgorithm>(&self.0, nonce, plaintext).map_err(Into::into)
    }

    pub fn decrypt(
        &self,
        ciphertext: &[u8],
        nonce: &Nonce<GatewayEncryptionAlgorithm>,
    ) -> Result<Vec<u8>, SharedKeyUsageError> {
        aead::decrypt::<GatewayEncryptionAlgorithm>(&self.0, nonce, ciphertext).map_err(Into::into)
    }
}

impl PemStorableKey for SharedSymmetricKey {
    type Error = SharedKeyConversionError;

    fn pem_type() -> &'static str {
        "AES-256-GCM-SIV GATEWAY SHARED KEY"
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_bytes()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
        Self::try_from_bytes(bytes)
    }
}
