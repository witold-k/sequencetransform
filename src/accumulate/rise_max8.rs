// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::compoundnumber::U8x8;

/// Collect the indices of input rows containing exactly 1.0 at each column.
/// All rows must have the same length. Each column supports up to eight matches.
/// Row index 255 is reserved as U8x8::FREE.
pub fn rise_max8<I, R>(rows: I) -> Vec<U8x8>
where
    I: IntoIterator<Item = R>,
    R: AsRef<[f32]>,
{
    let mut result: Vec<U8x8> = Vec::new();
    let mut width = None;
    for (row_index, row) in rows.into_iter().enumerate() {
        assert!(row_index < U8x8::FREE as usize, "rise_max8: row index 255 is reserved");
        let row = row.as_ref();
        match width {
            None => {
                width = Some(row.len());
                result.resize(row.len(), U8x8::default());
            }
            Some(expected) => assert_eq!(row.len(), expected, "rise_max8: inconsistent row width"),
        }
        for (column, &value) in row.iter().enumerate() {
            if value == 1.0 {
                assert!(result[column].push(row_index as u8), "rise_max8: more than eight matches in a column");
            }
        }
    }
    result
}
