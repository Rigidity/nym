// Copyright 2022-2024 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

use sqlx::FromRow;

#[derive(Clone)]
pub struct CoconutCredential {
    #[allow(dead_code)]
    pub id: i64,
    pub voucher_value: String,
    pub voucher_info: String,
    pub serial_number: String,
    pub binding_number: String,
    pub signature: String,
    pub epoch_id: String,
    pub consumed: bool,
}

#[derive(FromRow)]
pub struct StoredIssuedCredential {
    #[allow(dead_code)]
    pub id: i64,

    pub serial_number: String,
    pub binding_number: String,

    pub signature: String,

    pub variant_type: String,
    pub serialized_variant_data: String,

    pub epoch_id: String,
    pub consumed: bool,
}
