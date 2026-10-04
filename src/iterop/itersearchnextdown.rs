// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use num_traits::cast::FromPrimitive;
use crate::Number;
use crate::iterop::IterSearchNextDown;

// ---------------------------------------------------------------------------

impl<ITEM, ITER> IterSearchNextDown<ITER, ITEM>
where
    ITEM: FromPrimitive,
    ITER: Clone
{
    #[inline(always)]
    pub fn new(iter: ITER) -> IterSearchNextDown<ITER, ITEM> {
        let delta: ITEM = FromPrimitive::from_u32(1u32).unwrap();
        IterSearchNextDown::<ITER, ITEM> { iter, delta, count: 0 }
    }

    #[inline(always)]
    pub fn clone_inner(&self) -> ITER {
        self.iter.clone()
    }

    #[inline(always)]
    pub fn move_inner(self) -> ITER {
        self.iter
    }
}

impl<'a, ITEM, ITER> Iterator for IterSearchNextDown<ITER, ITEM>
where
    ITEM: 'a + Number<ITEM>,
    ITER: Iterator<Item = &'a ITEM> + Clone
{
    type Item = &'a ITEM;

    fn next(&mut self) -> Option<Self::Item> {
        let mut it = self.iter.clone();
        let delta: ITEM = self.delta;

        let opt_val = it.next();
        let mut prev_val = *match opt_val {
            None => {
                self.count = 0;
                return None;
            },
            Some(prev_val) => prev_val
        };
        let mut count: usize = 1;

        loop {
           let opt_val = it.next();
           let val = *match opt_val {
                None => {
                    self.count = 0;
                    count = count.saturating_sub(1);
                    self.count = count;
                    let opt_val = self.iter.nth(count);
                    return opt_val;
                },
                Some(val) => val
            };
            if prev_val - val >= delta {
                count = count.saturating_sub(1);
                self.count = count;
                let opt_val = self.iter.nth(count);
                return opt_val;
            }
            prev_val = val;
            count += 1;
        }
    }
}

