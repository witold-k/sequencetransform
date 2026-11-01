// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use num_traits::cast::FromPrimitive;
use crate::Number;
use crate::iterop::IterTillUp;

// ---------------------------------------------------------------------------

impl<'a, ITER, ITEM> IterTillUp<ITER, ITEM>
where
    ITEM: 'a + Clone + FromPrimitive,
    ITER: Iterator<Item = &'a ITEM> + Clone
{
    pub fn new(mut iter: ITER) -> IterTillUp<ITER, ITEM> {
        let null:  ITEM = FromPrimitive::from_u32(0u32).unwrap();
        let delta: ITEM = FromPrimitive::from_u32(1u32).unwrap();
        let opt_val = iter.next();
        let last_val = match opt_val {
            None      => &null,
            Some(val) => val
        };

        IterTillUp::<ITER, ITEM> { iter, delta, last_val: (*last_val).clone() }
    }

    #[inline(always)]
    pub fn clone_inner(self) -> ITER {
        self.iter.clone()
    }
}

impl<'a, ITER, ITEM> Iterator for IterTillUp<ITER, ITEM>
where
    ITEM: 'a + Number<ITEM>,
    ITER: Iterator<Item = &'a ITEM>
{
    type Item = &'a ITEM;

    fn next(&mut self) -> Option<Self::Item> {
        let opt_val = self.iter.next();
        let val = *match opt_val {
            None => {
                return None;
            },
            Some(val) => val
        };
        let last_val = self.last_val;
        self.last_val = val;
        if val - last_val < self.delta {
            return opt_val
        }

        None
    }
}

