// Copyright 2022-2024 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

//! Collection of initialization steps used by client implementations

use crate::client::key_manager::persistence::KeyStore;
use crate::client::key_manager::KeyManager;
use crate::error::ClientCoreError;
use crate::init::helpers::{
    choose_gateway_by_latency, get_specified_gateway, uniformly_random_gateway,
};
use crate::init::types::{
    GatewaySelectionSpecification, GatewaySetup, InitialisationResult, SelectedGateway,
};
use nym_client_core_gateways_storage::GatewaysDetailsStore;
use nym_client_core_gateways_storage::{GatewayDetails, GatewayRegistration};
use nym_gateway_client::client::InitGatewayClient;
use nym_topology::gateway;
use rand::rngs::OsRng;
use rand::{CryptoRng, RngCore};
use serde::Serialize;
use std::sync::Arc;

pub mod helpers;
pub mod types;

// helpers for error wrapping
async fn _store_gateway_details<D>(
    details_store: &D,
    details: &GatewayDetails,
) -> Result<(), ClientCoreError>
where
    D: GatewaysDetailsStore,
    D::StorageError: Send + Sync + 'static,
{
    todo!()
    // details_store
    //     .store_gateway_details(details)
    //     .await
    //     .map_err(|source| ClientCoreError::GatewaysDetailsStoreError {
    //         source: Box::new(source),
    //     })
}

async fn _load_active_gateway_details<D>(
    details_store: &D,
) -> Result<GatewayRegistration, ClientCoreError>
where
    D: GatewaysDetailsStore,
    D::StorageError: Send + Sync + 'static,
{
    details_store
        .active_gateway()
        .await
        .map_err(|source| ClientCoreError::GatewayDetailsStoreError {
            source: Box::new(source),
        })?
        .ok_or(ClientCoreError::NoActiveGatewaySet)
}

async fn _load_gateway_details<D>(
    details_store: &D,
    gateway_id: &str,
) -> Result<GatewayRegistration, ClientCoreError>
where
    D: GatewaysDetailsStore,
    D::StorageError: Send + Sync + 'static,
{
    details_store
        .load_gateway_details(gateway_id)
        .await
        .map_err(|source| ClientCoreError::UnavailableGatewayDetails {
            gateway_id: gateway_id.to_string(),
            source: Box::new(source),
        })
}

async fn _has_gateway_details<D>(
    details_store: &D,
    gateway_id: &str,
) -> Result<bool, ClientCoreError>
where
    D: GatewaysDetailsStore,
    D::StorageError: Send + Sync + 'static,
{
    details_store
        .has_gateway_details(gateway_id)
        .await
        .map_err(|source| ClientCoreError::GatewayDetailsStoreError {
            source: Box::new(source),
        })
}

async fn _load_client_keys<K>(key_store: &K) -> Result<KeyManager, ClientCoreError>
where
    K: KeyStore,
    K::StorageError: Send + Sync + 'static,
{
    KeyManager::load_keys(key_store)
        .await
        .map_err(|source| ClientCoreError::KeyStoreError {
            source: Box::new(source),
        })
}

pub async fn generate_new_client_keys<K, R>(
    rng: &mut R,
    key_store: &K,
) -> Result<(), ClientCoreError>
where
    R: RngCore + CryptoRng,
    K: KeyStore,
    K::StorageError: Send + Sync + 'static,
{
    KeyManager::generate_new(rng)
        .persist_keys(key_store)
        .await
        .map_err(|source| ClientCoreError::KeyStoreError {
            source: Box::new(source),
        })
}

// fn ensure_valid_details(
//     details: &PersistedGatewayDetails,
//     loaded_keys: &ManagedKeys,
// ) -> Result<(), ClientCoreError> {
//     details.validate(loaded_keys.gateway_shared_key().as_deref())
// }

async fn setup_new_gateway<K, D>(
    key_store: &K,
    details_store: &D,
    overwrite_data: bool,
    selection_specification: GatewaySelectionSpecification,
    available_gateways: Vec<gateway::Node>,
) -> Result<InitialisationResult, ClientCoreError>
where
    K: KeyStore,
    D: GatewaysDetailsStore,
    K::StorageError: Send + Sync + 'static,
    D::StorageError: Send + Sync + 'static,
{
    log::trace!("Setting up new gateway");

    // if we're setting up new gateway, we must have had generated long-term client keys before
    let client_keys = _load_client_keys(key_store).await?;

    // if we're setting up new gateway, failing to load existing information is fine.
    // as a matter of fact, it's only potentially a problem if we DO succeed

    todo!("check gateway details (maybe not even needed anymore, idk)");
    // if _load_gateway_details(details_store).await.is_ok() && !overwrite_data {
    //     return Err(ClientCoreError::ForbiddenKeyOverwrite);
    // }

    let mut rng = OsRng;

    let selected_gateway = match selection_specification {
        GatewaySelectionSpecification::UniformRemote { must_use_tls } => {
            let gateway = uniformly_random_gateway(&mut rng, &available_gateways, must_use_tls)?;
            SelectedGateway::from_topology_node(gateway, must_use_tls)?
        }
        GatewaySelectionSpecification::RemoteByLatency { must_use_tls } => {
            let gateway =
                choose_gateway_by_latency(&mut rng, &available_gateways, must_use_tls).await?;
            SelectedGateway::from_topology_node(gateway, must_use_tls)?
        }
        GatewaySelectionSpecification::Specified {
            must_use_tls,
            identity,
        } => {
            let gateway = get_specified_gateway(&identity, &available_gateways, must_use_tls)?;
            SelectedGateway::from_topology_node(gateway, must_use_tls)?
        }
        GatewaySelectionSpecification::Custom {
            gateway_identity,
            additional_data,
        } => SelectedGateway::custom(gateway_identity, additional_data)?,
    };

    // check if we already have details associated with this particular gateway
    // and if so, see if we can overwrite it
    let selected_id = selected_gateway.gateway_id().to_base58_string();
    if _has_gateway_details(details_store, &selected_id).await? && !overwrite_data {
        return Err(ClientCoreError::ForbiddenGatewayKeyOverwrite {
            gateway_id: selected_id,
        });
    }

    let (gateway_details, client) = match selected_gateway {
        SelectedGateway::Remote {
            gateway_id,
            gateway_owner_address,
            gateway_listener,
        } => {
            // if we're using a 'normal' gateway setup, do register
            let our_identity = client_keys.identity_keypair();
            let registration =
                helpers::register_with_gateway(gateway_id, gateway_listener, our_identity).await?;
            (
                GatewayDetails::new_remote(
                    gateway_id,
                    registration.shared_keys,
                    gateway_owner_address,
                    gateway_listener,
                ),
                Some(registration.authenticated_ephemeral_client),
            )
        }
        SelectedGateway::Custom {
            gateway_id,
            additional_data,
        } => (
            GatewayDetails::new_custom(gateway_id, additional_data),
            None,
        ),
    };

    // persist gateway details
    _store_gateway_details(details_store, &gateway_details).await?;

    Ok(InitialisationResult {
        gateway_registration: gateway_details.into(),
        client_keys,
        authenticated_ephemeral_client: client,
    })
}

async fn use_loaded_gateway_details<K, D>(
    key_store: &K,
    details_store: &D,
    gateway_id: Option<String>,
) -> Result<InitialisationResult, ClientCoreError>
where
    K: KeyStore,
    D: GatewaysDetailsStore,
    K::StorageError: Send + Sync + 'static,
    D::StorageError: Send + Sync + 'static,
{
    let loaded_details = if let Some(gateway_id) = gateway_id {
        _load_gateway_details(details_store, &gateway_id).await?
    } else {
        _load_active_gateway_details(details_store).await?
    };

    let loaded_keys = _load_client_keys(key_store).await?;

    // no need to persist anything as we got everything from the storage
    Ok(InitialisationResult::new_loaded(
        loaded_details,
        loaded_keys,
    ))
}

fn reuse_gateway_connection(
    authenticated_ephemeral_client: InitGatewayClient,
    gateway_registration: GatewayRegistration,
    client_keys: KeyManager,
) -> InitialisationResult {
    InitialisationResult {
        gateway_registration,
        client_keys,
        authenticated_ephemeral_client: Some(authenticated_ephemeral_client),
    }
}

pub async fn setup_gateway<K, D>(
    setup: GatewaySetup,
    key_store: &K,
    details_store: &D,
) -> Result<InitialisationResult, ClientCoreError>
where
    K: KeyStore,
    D: GatewaysDetailsStore,
    K::StorageError: Send + Sync + 'static,
    D::StorageError: Send + Sync + 'static,
{
    log::debug!("Setting up gateway");
    match setup {
        GatewaySetup::MustLoad { gateway_id } => {
            use_loaded_gateway_details(key_store, details_store, gateway_id).await
        }
        GatewaySetup::New {
            specification,
            available_gateways,
            overwrite_data,
        } => {
            setup_new_gateway(
                key_store,
                details_store,
                overwrite_data,
                specification,
                available_gateways,
            )
            .await
        }
        GatewaySetup::ReuseConnection {
            authenticated_ephemeral_client,
            gateway_details,
            managed_keys,
        } => Ok(reuse_gateway_connection(
            authenticated_ephemeral_client,
            gateway_details,
            managed_keys,
        )),
    }
}

pub fn output_to_json<T: Serialize>(init_results: &T, output_file: &str) {
    match std::fs::File::create(output_file) {
        Ok(file) => match serde_json::to_writer_pretty(file, init_results) {
            Ok(_) => println!("Saved: {output_file}"),
            Err(err) => eprintln!("Could not save {output_file}: {err}"),
        },
        Err(err) => eprintln!("Could not save {output_file}: {err}"),
    }
}
