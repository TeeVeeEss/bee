// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A module providing all types required to compute and maintain the ledger state.

pub mod snapshot;

mod balance;
mod balance_diff;
mod consumed_output;
mod created_output;
mod error;
mod ledger_index;
mod migration;
mod output_diff;
mod receipt;
mod treasury_diff;
mod treasury_output;
mod unspent;

pub use self::{
    balance::Balance,
    balance_diff::{BalanceDiff, BalanceDiffs},
    consumed_output::ConsumedOutput,
    created_output::CreatedOutput,
    error::Error,
    ledger_index::LedgerIndex,
    migration::Migration,
    output_diff::OutputDiff,
    receipt::Receipt,
    treasury_diff::TreasuryDiff,
    treasury_output::TreasuryOutput,
    unspent::Unspent,
};
