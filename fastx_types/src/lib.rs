// Copyright (c) Mysten Labs
// SPDX-License-Identifier: Apache-2.0
#![warn(
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    rust_2021_compatibility
)]
#![deny(warnings)]

use move_core_types::account_address::AccountAddress;

#[macro_use]
pub mod error;

pub mod base_types;
pub mod committee;
pub mod messages;
pub mod object;
pub mod serialize;
pub mod storage;

/// 0x1-- account address where Move stdlib modules are stored
pub const MOVE_STDLIB_ADDRESS: AccountAddress = AccountAddress::new([
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8,
]);

/// 0x2-- account address where fastX framework modules are stored
pub const FASTX_FRAMEWORK_ADDRESS: AccountAddress = AccountAddress::new([
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8,
]);
