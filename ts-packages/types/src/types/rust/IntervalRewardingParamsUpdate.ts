// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { RewardedSetParams } from './RewardedSetParams';

/**
 * Specification on how the rewarding params should be updated.
 */
export type IntervalRewardingParamsUpdate = {
  /**
   * Defines the new value of the reward pool.
   */
  reward_pool: string | null;
  /**
   * Defines the new value of the staking supply.
   */
  staking_supply: string | null;
  /**
   * Defines the new value of the staking supply scale factor.
   */
  staking_supply_scale_factor: string | null;
  /**
   * Defines the new value of the sybil resistance percent.
   */
  sybil_resistance_percent: string | null;
  /**
   * Defines the new value of the active set work factor.
   */
  active_set_work_factor: string | null;
  /**
   * Defines the new value of the interval pool emission rate.
   */
  interval_pool_emission: string | null;
  /**
   * Defines the parameters of the rewarded set.
   */
  rewarded_set_params: RewardedSetParams | null;
};
