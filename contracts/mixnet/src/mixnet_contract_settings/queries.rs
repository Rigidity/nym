// Copyright 2021-2023 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

use super::storage;
use crate::mixnet_contract_settings::storage::ADMIN;
use cosmwasm_std::{Deps, StdResult};
use cw_controllers::AdminResponse;
use mixnet_contract_common::{ContractBuildInformation, ContractState, ContractStateParams};
use nym_contracts_common::get_build_information;

pub(crate) fn query_admin(deps: Deps<'_>) -> StdResult<AdminResponse> {
    ADMIN.query_admin(deps)
}

pub(crate) fn query_contract_state(deps: Deps<'_>) -> StdResult<ContractState> {
    storage::CONTRACT_STATE.load(deps.storage)
}

pub(crate) fn query_contract_settings_params(deps: Deps<'_>) -> StdResult<ContractStateParams> {
    storage::CONTRACT_STATE
        .load(deps.storage)
        .map(|settings| settings.params)
}

pub(crate) fn query_rewarding_validator_address(deps: Deps<'_>) -> StdResult<String> {
    storage::CONTRACT_STATE
        .load(deps.storage)
        .map(|settings| settings.rewarding_validator_address.to_string())
}

pub(crate) fn query_contract_version() -> ContractBuildInformation {
    get_build_information!()
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::support::tests::test_helpers;
    use cosmwasm_std::{coin, Addr};
    use mixnet_contract_common::{ConfigScoreParams, DelegationsParams, OperatorsParams};

    #[test]
    fn query_for_contract_settings_works() {
        let mut deps = test_helpers::init_contract();

        #[allow(deprecated)]
        let dummy_state = ContractState {
            owner: Some(Addr::unchecked("foomp")),
            rewarding_validator_address: Addr::unchecked("monitor"),
            vesting_contract_address: Addr::unchecked("foomp"),
            rewarding_denom: "unym".to_string(),
            params: ContractStateParams {
                delegations_params: DelegationsParams {
                    minimum_delegation: None,
                },
                operators_params: OperatorsParams {
                    minimum_pledge: coin(123u128, "unym"),
                    profit_margin: Default::default(),
                    interval_operating_cost: Default::default(),
                },
                config_score_params: ConfigScoreParams {
                    current_nym_node_semver: "1.1.10".to_string(),
                    version_weights: Default::default(),
                    version_score_formula_params: Default::default(),
                },
            },
        };

        storage::CONTRACT_STATE
            .save(deps.as_mut().storage, &dummy_state)
            .unwrap();

        assert_eq!(
            dummy_state.params,
            query_contract_settings_params(deps.as_ref()).unwrap()
        )
    }

    #[test]
    fn query_for_contract_version_works() {
        // this basically means _something_ was grabbed from the environment at compilation time
        let version = query_contract_version();
        assert!(!version.build_timestamp.is_empty());
        assert!(!version.build_version.is_empty());
        assert!(!version.commit_sha.is_empty());
        assert!(!version.commit_timestamp.is_empty());
        assert!(!version.commit_branch.is_empty());
        assert!(!version.rustc_version.is_empty());
    }
}
