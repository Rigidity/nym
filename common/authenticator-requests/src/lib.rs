// Copyright 2024 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

pub mod traits;
pub mod v1;
pub mod v2;
pub mod v3;
pub mod v4;

mod error;

pub use error::Error;
pub use v4 as latest;

pub const CURRENT_VERSION: u8 = 4;

fn make_bincode_serializer() -> impl bincode::Options {
    use bincode::Options;
    bincode::DefaultOptions::new()
        .with_big_endian()
        .with_varint_encoding()
}
