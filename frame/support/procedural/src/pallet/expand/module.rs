// This file is part of Substrate.

// Copyright (C) 2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

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

use crate::pallet::Def;

/// * Add derive Eq, PartialEq, Debug and Clone on Module
pub fn expand_module(def: &mut Def) -> proc_macro2::TokenStream {
	let scrate = &def.scrate();

	let module_item = {
		let item = &mut def.item.content.as_mut().expect("Checked by def").1[def.module.index];
		if let syn::Item::Struct(item) = item {
			item
		} else {
			unreachable!("Checked by module parser")
		}
	};

	module_item.attrs.push(syn::parse_quote!(
		#[derive(
			#scrate::CloneNoBound,
			#scrate::EqNoBound,
			#scrate::PartialEqNoBound,
			#scrate::DebugStripped,
		)]
	));

	Default::default()
}
