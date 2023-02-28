// SPDX-License-Identifier: Apache-2.0
// This file is part of Frontier.
//
// Copyright (c) 2020-2022 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use core::marker::PhantomData;
use fp_evm::{PrecompileHandle, Precompile, PrecompileResult, ExitSucceed};
use frame_support::traits::tokens::fungibles::Inspect;
use precompile_utils::prelude::*;

#[precompile_utils::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum ERC20Methods {
    TotalSupply = "totalSupply()",
	BalanceOf = "balanceOf(address)",
	Allowance = "allowance(address,address)",
	Transfer = "transfer(address,uint256)",
	Approve = "approve(address,uint256)",
	TransferFrom = "transferFrom(address,address,uint256)",
	Name = "name()",
	Symbol = "symbol()",
	Decimals = "decimals()",
}

pub struct Fungibles<R>(PhantomData<R>);

impl<R> Precompile for Fungibles<R>
where
    R: pallet_evmless::Config,
    AssetIdOf<R>: From<u32>,
    BalanceOf<R>: EvmData,
{
    fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        // todo: check address
        //let address = handle.code_address();

        let selector = match handle.read_selector() {
            Ok(selector) => selector,
            Err(e) => return Err(e.into()),
        };

        if let Err(err) = handle.check_function_modifier(match selector {
            ERC20Methods::Approve | ERC20Methods::Transfer | ERC20Methods::TransferFrom => FunctionModifier::NonPayable,
            _ => FunctionModifier::View,
        }) {
            return Err(err.into());
        }

        // todo: change to appropriate method implementations
        match selector {
            ERC20Methods::TotalSupply => Self::total_supply(handle),
            ERC20Methods::BalanceOf => Self::total_supply(handle),
            ERC20Methods::Allowance => Self::total_supply(handle),
            ERC20Methods::Transfer => Self::total_supply(handle),
            ERC20Methods::Approve => Self::total_supply(handle),
            ERC20Methods::TransferFrom => Self::total_supply(handle),
            ERC20Methods::Name => Self::total_supply(handle),
            ERC20Methods::Symbol => Self::total_supply(handle),
            ERC20Methods::Decimals => Self::total_supply(handle),
        }
    }
}

pub type AssetIdOf<R> =
        <<R as pallet_evmless::Config>::Fungibles as Inspect<<R as frame_system::Config>::AccountId>>::AssetId;

pub type BalanceOf<R> =
        <<R as pallet_evmless::Config>::Fungibles as Inspect<<R as frame_system::Config>::AccountId>>::Balance;

impl<R> Fungibles<R>
where
    R: pallet_evmless::Config,
    AssetIdOf<R>: From<u32>,
    BalanceOf<R>: EvmData,
{
    fn total_supply(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
        handle.record_cost(RuntimeHelper::<R>::db_read_gas_cost())?;

        let t = R::Fungibles::total_issuance(0u32.into());

        Ok(PrecompileOutput {
            exit_status: ExitSucceed::Returned,
            output: EvmDataWriter::new().write(t).build()
        })
    }
}