// Copyright 2024 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

use crate::{GatewayRequestsError, SimpleGatewayRequestsError, SymmetricKey};
use serde::{Deserialize, Serialize};
use tungstenite::Message;

// naming things is difficult...
// the name implies that the content is encrypted before being sent on the wire
#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
pub enum SensitiveServerResponse {
    KeyUpgradeAck {},
}

impl SensitiveServerResponse {
    pub fn encrypt<S: SymmetricKey>(
        &self,
        key: &S,
    ) -> Result<ServerResponse, GatewayRequestsError> {
        // we're using json representation for few reasons:
        // - ease of re-implementation in other languages (compared to for example bincode)
        // - we expect all requests to be relatively small - for anything bigger use BinaryRequest!
        // - the schema is self-describing which simplifies deserialisation

        // SAFETY: the trait has been derived correctly with no weird variants
        let plaintext = serde_json::to_vec(self).unwrap();
        let nonce = key.random_nonce_or_iv();
        let ciphertext = key.encrypt(&plaintext, Some(&nonce))?;
        Ok(ServerResponse::EncryptedResponse { ciphertext, nonce })
    }

    pub fn decrypt<S: SymmetricKey>(
        ciphertext: &[u8],
        nonce: &[u8],
        key: &S,
    ) -> Result<Self, GatewayRequestsError> {
        let plaintext = key.decrypt(ciphertext, Some(nonce))?;
        serde_json::from_slice(&plaintext)
            .map_err(|source| GatewayRequestsError::MalformedRequest { source })
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
#[non_exhaustive]
pub enum ServerResponse {
    Authenticate {
        #[serde(default)]
        protocol_version: Option<u8>,
        status: bool,
        bandwidth_remaining: i64,
    },
    Register {
        #[serde(default)]
        protocol_version: Option<u8>,
        status: bool,
    },
    EncryptedResponse {
        ciphertext: Vec<u8>,
        nonce: Vec<u8>,
    },
    Bandwidth {
        available_total: i64,
    },
    Send {
        remaining_bandwidth: i64,
    },
    SupportedProtocol {
        version: u8,
    },
    // Generic error
    Error {
        message: String,
    },
    // Specific typed errors
    // so that clients could match on this variant without doing naive string matching
    TypedError {
        error: SimpleGatewayRequestsError,
    },
}

impl ServerResponse {
    pub fn name(&self) -> String {
        match self {
            ServerResponse::Authenticate { .. } => "Authenticate".to_string(),
            ServerResponse::Register { .. } => "Register".to_string(),
            ServerResponse::Bandwidth { .. } => "Bandwidth".to_string(),
            ServerResponse::Send { .. } => "Send".to_string(),
            ServerResponse::Error { .. } => "Error".to_string(),
            ServerResponse::TypedError { .. } => "TypedError".to_string(),
            ServerResponse::SupportedProtocol { .. } => "SupportedProtocol".to_string(),
            ServerResponse::EncryptedResponse { .. } => "EncryptedResponse".to_string(),
        }
    }
    pub fn new_error<S: Into<String>>(msg: S) -> Self {
        ServerResponse::Error {
            message: msg.into(),
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, ServerResponse::Error { .. })
    }

    pub fn implies_successful_authentication(&self) -> bool {
        match self {
            ServerResponse::Authenticate { status, .. } => *status,
            ServerResponse::Register { status, .. } => *status,
            _ => false,
        }
    }
}

impl From<ServerResponse> for Message {
    fn from(res: ServerResponse) -> Self {
        // it should be safe to call `unwrap` here as the message is generated by the server
        // so if it fails (and consequently panics) it's a bug that should be resolved
        let str_res = serde_json::to_string(&res).unwrap();
        Message::Text(str_res)
    }
}

impl TryFrom<String> for ServerResponse {
    type Error = serde_json::Error;

    fn try_from(msg: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&msg)
    }
}