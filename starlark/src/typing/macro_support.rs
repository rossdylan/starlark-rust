/*
 * Copyright 2019 The Starlark in Rust Authors.
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![doc(hidden)]

use crate::typing::Ty;
use crate::typing::TyBasic;

pub fn unpack_args_item_ty(ty: Ty) -> Ty {
    Ty::unions(
        ty.iter_union()
            .iter()
            .map(|ty| match ty {
                TyBasic::Tuple(item) => item.item_ty(),
                _ => Ty::any(),
            })
            .collect(),
    )
}

pub fn unpack_kwargs_value_ty(ty: Ty) -> Ty {
    Ty::unions(
        ty.iter_union()
            .iter()
            .map(|ty| match ty {
                TyBasic::Dict(_, value) => value.to_ty(),
                _ => Ty::any(),
            })
            .collect(),
    )
}
