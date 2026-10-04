// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::selector::{ SimpleProcessor,  SimpleRefProcessor };
use crate::selector:: { Processor, Selector, SelectReadIterator, SelectWriteIterator, SelectorResult };

// ---------------------------------------------------------------------------

impl<'a, ITEM, RI, SELECT> Processor<'a, ITEM> for SimpleProcessor<RI, SELECT>
where
    RI: SelectReadIterator<'a, ITEM>,
    SELECT: Selector,
    ITEM: 'a + Clone
{
    /**
     * return amout of iteration
     */
    fn populate<'b, WI>(
        &mut self,
        wi: WI
    ) -> bool
    where
        'a: 'b,
        WI: 'b + SelectWriteIterator<'b, ITEM>,
        ITEM: Clone
    {
        let ri = self.read_iterator.clone();
        let result = self.selector.populate(ri, wi);
        let has_more = match result {
            SelectorResult::Ok       => true,
            SelectorResult::Error(_) => false
        };
        match self.read_iterator.next() {
            Some(_) => has_more,
            None    => false
        }
    }

    fn process_len(&self) -> usize {
        self.selector.process_len()
    }

}

impl<'a, 'r, ITEM: 'a, RI, SELECT: 'r> Processor<'a, ITEM> for SimpleRefProcessor<'r, RI, SELECT>
where
    RI: SelectReadIterator<'a, ITEM>,
    SELECT: Selector,
    ITEM: Clone
{
    /**
     * return amout of iteration
     */
    fn populate<'b, WI>(
        &mut self,
        wi: WI
    ) -> bool
    where
        'a: 'b,
        WI: 'b + SelectWriteIterator<'b, ITEM>,
        ITEM: Clone
    {
        let ri = self.read_iterator.clone();
        let result = self.selector.populate(ri, wi);
        let has_more = match result {
            SelectorResult::Ok       => true,
            SelectorResult::Error(_) => false
        };
        match self.read_iterator.next() {
            Some(_) => has_more,
            None    => false
        }
    }

    fn process_len(&self) -> usize {
        self.selector.process_len()
    }

}

