// Copyright 2021-2023 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: GPL-3.0-only

use super::reward_estimate::compute_reward_estimate;
use crate::node_status_api::models::{AxumErrorResponse, AxumResult};
use crate::storage::NymApiStorage;
use crate::support::caching::Cache;
use crate::{NodeStatusCache, NymContractCache};
use cosmwasm_std::Decimal;
use nym_api_requests::models::{
    ComputeRewardEstParam, GatewayBondAnnotated, GatewayCoreStatusResponse,
    GatewayStatusReportResponse, GatewayUptimeHistoryResponse, GatewayUptimeResponse,
    MixNodeBondAnnotated, MixnodeCoreStatusResponse, MixnodeStatus, MixnodeStatusReportResponse,
    MixnodeStatusResponse, MixnodeUptimeHistoryResponse, RewardEstimationResponse,
    StakeSaturationResponse, UptimeResponse,
};
use nym_mixnet_contract_common::NodeId;

pub(crate) enum RewardedSetStatus {
    Active,
    Standby,
    Inactive,
}

impl From<MixnodeStatus> for RewardedSetStatus {
    fn from(value: MixnodeStatus) -> Self {
        match value {
            MixnodeStatus::Active => RewardedSetStatus::Active,
            MixnodeStatus::Standby => RewardedSetStatus::Standby,
            // for all intents and purposes, missing node is treated as inactive for rewarding (since it wouldn't get anything
            MixnodeStatus::Inactive => RewardedSetStatus::Inactive,
            MixnodeStatus::NotFound => RewardedSetStatus::Inactive,
        }
    }
}

async fn gateway_identity_to_node_id(
    cache: &NodeStatusCache,
    identity: &str,
) -> AxumResult<NodeId> {
    let node_id = cache
        .map_identity_to_node_id(identity)
        .await
        .ok_or(AxumErrorResponse::not_found("gateway bond not found"))?;
    Ok(node_id)
}

async fn get_gateway_bond_annotated(
    cache: &NodeStatusCache,
    node_id: NodeId,
) -> AxumResult<GatewayBondAnnotated> {
    cache
        .gateway_annotated(node_id)
        .await
        .ok_or(AxumErrorResponse::not_found("gateway bond not found"))
}

async fn get_gateway_bond_annotated_by_identity(
    cache: &NodeStatusCache,
    identity: &str,
) -> AxumResult<GatewayBondAnnotated> {
    let node_id = gateway_identity_to_node_id(cache, identity).await?;
    get_gateway_bond_annotated(cache, node_id).await
}

async fn get_mixnode_bond_annotated(
    cache: &NodeStatusCache,
    mix_id: NodeId,
) -> AxumResult<MixNodeBondAnnotated> {
    cache
        .mixnode_annotated(mix_id)
        .await
        .ok_or(AxumErrorResponse::not_found("mixnode bond not found"))
}

pub(crate) async fn _gateway_report(
    cache: &NodeStatusCache,
    identity: &str,
) -> AxumResult<GatewayStatusReportResponse> {
    let gateway = get_gateway_bond_annotated_by_identity(cache, identity).await?;

    Ok(GatewayStatusReportResponse {
        identity: gateway.identity().to_owned(),
        owner: gateway.owner().to_string(),
        most_recent: gateway.node_performance.most_recent.round_to_integer(),
        last_hour: gateway.node_performance.last_hour.round_to_integer(),
        last_day: gateway.node_performance.last_24h.round_to_integer(),
    })
}

pub(crate) async fn _gateway_uptime_history(
    storage: &NymApiStorage,
    nym_contract_cache: &NymContractCache,
    identity: &str,
) -> AxumResult<GatewayUptimeHistoryResponse> {
    let history = storage
        .get_gateway_uptime_history_by_identity(identity)
        .await
        .map_err(AxumErrorResponse::not_found)?;

    let owner = nym_contract_cache
        .legacy_gateway_owner(history.node_id)
        .await
        .ok_or_else(|| AxumErrorResponse::not_found("could not determine gateway owner"))?;

    Ok(GatewayUptimeHistoryResponse {
        identity: history.identity,
        owner,
        history: history.history.into_iter().map(Into::into).collect(),
    })
}

pub(crate) async fn _gateway_core_status_count(
    storage: &NymApiStorage,
    identity: &str,
    since: Option<i64>,
) -> AxumResult<GatewayCoreStatusResponse> {
    let count = storage
        .get_core_gateway_status_count_by_identity(identity, since)
        .await
        .map_err(AxumErrorResponse::not_found)?;

    Ok(GatewayCoreStatusResponse {
        identity: identity.to_string(),
        count,
    })
}

pub(crate) async fn _mixnode_report(
    cache: &NodeStatusCache,
    mix_id: NodeId,
) -> AxumResult<MixnodeStatusReportResponse> {
    let mixnode = get_mixnode_bond_annotated(cache, mix_id).await?;

    Ok(MixnodeStatusReportResponse {
        mix_id,
        identity: mixnode.identity_key().to_owned(),
        owner: mixnode.owner().to_string(),
        most_recent: mixnode.node_performance.most_recent.round_to_integer(),
        last_hour: mixnode.node_performance.last_hour.round_to_integer(),
        last_day: mixnode.node_performance.last_24h.round_to_integer(),
    })
}

pub(crate) async fn _mixnode_uptime_history(
    storage: &NymApiStorage,
    nym_contract_cache: &NymContractCache,
    mix_id: NodeId,
) -> AxumResult<MixnodeUptimeHistoryResponse> {
    let history = storage
        .get_mixnode_uptime_history(mix_id)
        .await
        .map_err(AxumErrorResponse::not_found)?;

    let owner = nym_contract_cache
        .legacy_gateway_owner(mix_id)
        .await
        .ok_or_else(|| AxumErrorResponse::not_found("could not determine mixnode owner"))?;

    Ok(MixnodeUptimeHistoryResponse {
        mix_id,
        identity: history.identity,
        owner,
        history: history.history.into_iter().map(Into::into).collect(),
    })
}

pub(crate) async fn _mixnode_core_status_count(
    storage: &NymApiStorage,
    mix_id: NodeId,
    since: Option<i64>,
) -> AxumResult<MixnodeCoreStatusResponse> {
    let count = storage
        .get_core_mixnode_status_count(mix_id, since)
        .await
        .map_err(AxumErrorResponse::not_found)?;

    Ok(MixnodeCoreStatusResponse { mix_id, count })
}

pub(crate) async fn _get_mixnode_status(
    cache: &NymContractCache,
    mix_id: NodeId,
) -> MixnodeStatusResponse {
    MixnodeStatusResponse {
        status: cache.mixnode_status(mix_id).await,
    }
}

pub(crate) async fn _get_mixnode_reward_estimation(
    status_cache: &NodeStatusCache,
    contract_cache: &NymContractCache,
    mix_id: NodeId,
) -> AxumResult<RewardEstimationResponse> {
    let status = contract_cache.mixnode_status(mix_id).await;
    let mixnode = status_cache
        .mixnode_annotated(mix_id)
        .await
        .ok_or_else(|| AxumErrorResponse::not_found("mixnode bond not found"))?;

    let reward_params = contract_cache.interval_reward_params().await;
    let as_at = reward_params.timestamp();
    let reward_params = reward_params
        .into_inner()
        .ok_or_else(AxumErrorResponse::internal)?;
    let current_interval = contract_cache
        .current_interval()
        .await
        .into_inner()
        .ok_or_else(AxumErrorResponse::internal)?;

    let reward_estimation = compute_reward_estimate(
        &mixnode.mixnode_details,
        mixnode.performance,
        status.into(),
        reward_params,
        current_interval,
    );

    Ok(RewardEstimationResponse {
        estimation: reward_estimation,
        reward_params,
        epoch: current_interval,
        as_at: as_at.unix_timestamp(),
    })
}

pub(crate) async fn _compute_mixnode_reward_estimation(
    user_reward_param: &ComputeRewardEstParam,
    status_cache: &NodeStatusCache,
    contract_cache: &NymContractCache,
    mix_id: NodeId,
) -> AxumResult<RewardEstimationResponse> {
    let mut mixnode = status_cache
        .mixnode_annotated(mix_id)
        .await
        .ok_or_else(|| AxumErrorResponse::not_found("mixnode bond not found"))?;

    let reward_params = contract_cache.interval_reward_params().await;
    let as_at = reward_params.timestamp();
    let reward_params = reward_params
        .into_inner()
        .ok_or_else(AxumErrorResponse::internal)?;
    let current_interval = contract_cache
        .current_interval()
        .await
        .into_inner()
        .ok_or_else(AxumErrorResponse::internal)?;

    // For these parameters we either use the provided ones, or fall back to the system ones
    let performance = user_reward_param.performance.unwrap_or(mixnode.performance);

    let status = match user_reward_param.active_in_rewarded_set {
        Some(true) => RewardedSetStatus::Active,
        Some(false) => RewardedSetStatus::Standby,
        None => {
            let actual_status = contract_cache.mixnode_status(mix_id).await;
            actual_status.into()
        }
    };

    if let Some(pledge_amount) = user_reward_param.pledge_amount {
        mixnode.mixnode_details.rewarding_details.operator =
            Decimal::from_ratio(pledge_amount, 1u64);
    }
    if let Some(total_delegation) = user_reward_param.total_delegation {
        mixnode.mixnode_details.rewarding_details.delegates =
            Decimal::from_ratio(total_delegation, 1u64);
    }

    if let Some(profit_margin_percent) = user_reward_param.profit_margin_percent {
        mixnode
            .mixnode_details
            .rewarding_details
            .cost_params
            .profit_margin_percent = profit_margin_percent;
    }

    if let Some(interval_operating_cost) = &user_reward_param.interval_operating_cost {
        mixnode
            .mixnode_details
            .rewarding_details
            .cost_params
            .interval_operating_cost = interval_operating_cost.clone();
    }

    if mixnode.mixnode_details.rewarding_details.operator
        + mixnode.mixnode_details.rewarding_details.delegates
        > reward_params.interval.staking_supply
    {
        return Err(AxumErrorResponse::unprocessable_entity(
            "Pledge plus delegation too large",
        ));
    }

    let reward_estimation = compute_reward_estimate(
        &mixnode.mixnode_details,
        performance,
        status,
        reward_params,
        current_interval,
    );

    Ok(RewardEstimationResponse {
        estimation: reward_estimation,
        reward_params,
        epoch: current_interval,
        as_at: as_at.unix_timestamp(),
    })
}

pub(crate) async fn _get_mixnode_stake_saturation(
    status_cache: &NodeStatusCache,
    contract_cache: &NymContractCache,
    mix_id: NodeId,
) -> AxumResult<StakeSaturationResponse> {
    let mixnode = status_cache
        .mixnode_annotated(mix_id)
        .await
        .ok_or_else(|| AxumErrorResponse::not_found("mixnode bond not found"))?;

    // Recompute the stake saturation just so that we can confidently state that the `as_at`
    // field is consistent and correct. Luckily this is very cheap.
    let reward_params = contract_cache.interval_reward_params().await;
    let as_at = reward_params.timestamp();
    let rewarding_params = reward_params
        .into_inner()
        .ok_or_else(AxumErrorResponse::internal)?;

    Ok(StakeSaturationResponse {
        saturation: mixnode
            .mixnode_details
            .rewarding_details
            .bond_saturation(&rewarding_params),
        uncapped_saturation: mixnode
            .mixnode_details
            .rewarding_details
            .uncapped_bond_saturation(&rewarding_params),
        as_at: as_at.unix_timestamp(),
    })
}

#[allow(deprecated)]
pub(crate) async fn _get_mixnode_inclusion_probability(
    cache: &NodeStatusCache,
    mix_id: NodeId,
) -> AxumResult<nym_api_requests::models::InclusionProbabilityResponse> {
    cache
        .inclusion_probabilities()
        .await
        .map(Cache::into_inner)
        .and_then(|p| p.node(mix_id).cloned())
        .map(|p| nym_api_requests::models::InclusionProbabilityResponse {
            in_active: p.in_active.into(),
            in_reserve: p.in_reserve.into(),
        })
        .ok_or_else(|| AxumErrorResponse::not_found("mixnode bond not found"))
}

pub(crate) async fn _get_mixnode_avg_uptime(
    cache: &NodeStatusCache,
    mix_id: NodeId,
) -> AxumResult<UptimeResponse> {
    let mixnode = get_mixnode_bond_annotated(cache, mix_id).await?;

    Ok(UptimeResponse {
        mix_id,
        avg_uptime: mixnode.node_performance.last_24h.round_to_integer(),
        node_performance: mixnode.node_performance,
    })
}

pub(crate) async fn _get_gateway_avg_uptime(
    cache: &NodeStatusCache,
    identity: &str,
) -> AxumResult<GatewayUptimeResponse> {
    let gateway = get_gateway_bond_annotated_by_identity(cache, identity).await?;

    Ok(GatewayUptimeResponse {
        identity: identity.to_string(),
        avg_uptime: gateway.node_performance.last_24h.round_to_integer(),
        node_performance: gateway.node_performance,
    })
}

#[allow(deprecated)]
pub(crate) async fn _get_mixnode_inclusion_probabilities(
    cache: &NodeStatusCache,
) -> AxumResult<nym_api_requests::models::AllInclusionProbabilitiesResponse> {
    if let Some(prob) = cache.inclusion_probabilities().await {
        let as_at = prob.timestamp();
        let prob = prob.into_inner();
        Ok(
            nym_api_requests::models::AllInclusionProbabilitiesResponse {
                inclusion_probabilities: prob.inclusion_probabilities,
                samples: prob.samples,
                elapsed: prob.elapsed,
                delta_max: prob.delta_max,
                delta_l2: prob.delta_l2,
                as_at: as_at.unix_timestamp(),
            },
        )
    } else {
        Err(AxumErrorResponse::service_unavailable())
    }
}

pub(crate) async fn _get_legacy_mixnodes_detailed(
    cache: &NodeStatusCache,
) -> Vec<MixNodeBondAnnotated> {
    cache
        .mixnodes_annotated_filtered()
        .await
        .unwrap_or_default()
}

pub(crate) async fn _get_mixnodes_detailed_unfiltered(
    cache: &NodeStatusCache,
) -> Vec<MixNodeBondAnnotated> {
    cache.mixnodes_annotated_full().await.unwrap_or_default()
}

pub(crate) async fn _get_rewarded_set_legacy_mixnodes_detailed(
    status_cache: &NodeStatusCache,
    contract_cache: &NymContractCache,
) -> Vec<MixNodeBondAnnotated> {
    let Some(rewarded_set) = contract_cache.rewarded_set().await else {
        return Vec::new();
    };
    let Some(mixnodes) = status_cache.mixnodes_annotated_full().await else {
        return Vec::new();
    };
    mixnodes
        .into_iter()
        .filter(|m| {
            rewarded_set.is_active_mixnode(&m.mix_id()) || rewarded_set.is_standby(&m.mix_id())
        })
        .collect()
}

pub(crate) async fn _get_active_set_legacy_mixnodes_detailed(
    status_cache: &NodeStatusCache,
    contract_cache: &NymContractCache,
) -> Vec<MixNodeBondAnnotated> {
    let Some(rewarded_set) = contract_cache.rewarded_set().await else {
        return Vec::new();
    };
    let Some(mixnodes) = status_cache.mixnodes_annotated_full().await else {
        return Vec::new();
    };
    mixnodes
        .into_iter()
        .filter(|m| rewarded_set.is_active_mixnode(&m.mix_id()))
        .collect()
}

pub(crate) async fn _get_legacy_gateways_detailed(
    cache: &NodeStatusCache,
) -> Vec<GatewayBondAnnotated> {
    cache
        .gateways_annotated_filtered()
        .await
        .unwrap_or_default()
}

pub(crate) async fn _get_legacy_gateways_detailed_unfiltered(
    cache: &NodeStatusCache,
) -> Vec<GatewayBondAnnotated> {
    cache.gateways_annotated_full().await.unwrap_or_default()
}
