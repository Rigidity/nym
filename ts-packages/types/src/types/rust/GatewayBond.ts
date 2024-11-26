// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DecCoin } from './DecCoin';
import type { Gateway } from './Gateway';

export type GatewayBond = {
  pledge_amount: DecCoin;
  owner: string;
  block_height: bigint;
  gateway: Gateway;
  proxy: string | null;
};
