// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.

/**
 * Basic information of a node that used to be part of the mix network but has already unbonded.
 */
export type UnbondedMixnode = {
  /**
   * Base58-encoded ed25519 EdDSA public key.
   */
  identity_key: string;
  /**
   * Address of the owner of this mixnode.
   */
  owner: string;
  /**
   * Entity who bonded this mixnode on behalf of the owner.
   * If exists, it's most likely the address of the vesting contract.
   */
  proxy: string | null;
  /**
   * Block height at which this mixnode has unbonded.
   */
  unbonding_height: number;
};
