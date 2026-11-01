// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use simplefield::{field::Field, orientation::RowMajor};

pub mod counting_window;

#[derive(Clone)]
pub struct CountingWindow<'a, T>
where
    T: Clone,
    Field<RowMajor, T>: Clone,
{
    #[allow(dead_code)]
    data: &'a [T],
    #[allow(dead_code)]
    field: Field<RowMajor, T>,
}

