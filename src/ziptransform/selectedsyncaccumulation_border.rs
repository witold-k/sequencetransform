// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use num_traits::cast::FromPrimitive;

use crate::ConvertToIdentity;
use crate::select::*;
use crate::iterop::*;
use crate::ziptransform::*;

impl<ITEMOUT> SelectedSyncAccumulationBorder<ITEMOUT> {
    pub fn new(border: ITEMOUT) -> Self {
        SelectedSyncAccumulationBorder { border }
    }
}

impl<ITEMOUT> SelectedSyncAccumulationBorder<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    #[inline(always)]
    fn transform_inline<'a, 'b, RI, ITEMIN, CTO>(
        &self,
        mut ri_data:    RI,
        mut ri_trigger: RI,
        out:            &mut [ITEMOUT],
        conv:           CTO
    ) -> Result<usize, TransformResult>
    where
        'a: 'b,
        RI:     TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    {
        let border = self.border;
        let len1 = ri_data.len();
        let len2 = ri_trigger.len();
        if len1 != len2 {
            return Err(TransformResult::Error(1));
        }
        let wlen = out.len();
        if wlen < len1  {
            return Err(TransformResult::Error(2));
        }

        let sel = Max::<ITEMIN>::default();

        let mut wi = out.iter_mut();
        let mut iup = IterSearchNextUp::<RI, ITEMIN>::new(ri_trigger.clone());
        loop {
            match iup.next() {
                None => {
                    return Ok(wlen - wi.len());
                },
                Some(_) => {
                    iup.count += 1;
                    for _ in 0 .. iup.count {
                        let we = match wi.next() {
                            Some(we) => we,
                            None     => {
                                return Ok(wlen - wi.len());

                            }
                        };
                        *we = FromPrimitive::from_u32(0u32).unwrap();
                    }

                    ri_trigger.nth(iup.count);
                    let mut idown = IterSearchNextDown::<RI, ITEMIN>::new(iup.clone_inner());
                    match idown.next() {
                        Some (_) => {
                            _ = ri_data.nth(iup.count);
                            let ri_datat = ri_data.clone();
                            _ = ri_data.nth(idown.count);
                            let ri_datat = ri_datat.take(idown.count + 1);
                            let max = sel.select(ri_datat, ConvertToIdentity::<ITEMIN>::default());
                            let val_out: ITEMOUT = conv.convert(max.item);
                            let val_out = if val_out >= border { FromPrimitive::from_u32(1).unwrap() } else { FromPrimitive::from_u32(0).unwrap() };
                            idown.count += 1;
                            for _ in 0 .. idown.count {
                                let we = match wi.next() {
                                    Some(we) => we,
                                    None     => {
                                        return Ok(wlen - wi.len());

                                    }
                                };
                                *we = val_out;
                            }
                        },
                        None => {
                            return Ok(wlen - wi.len());
                        }
                    };
                    iup = IterSearchNextUp::<RI, ITEMIN>::new(idown.move_inner());
                    for _ in 0 .. iup.count {
                        let we = match wi.next() {
                            Some(we) => we,
                            None     => {
                                return Ok(wlen - wi.len());

                            }
                        };
                        *we = FromPrimitive::from_u32(0u32).unwrap();
                    }
                }
            };
        }
    }

    pub fn transform_self<'a, RI, ITEMIN, CTO>(
        &self,
        ri1:         RI,
        ri2:         RI,
        out:         &mut [ITEMOUT],
        conv:        CTO
    ) -> Result<usize, TransformResult>
    where
        RI:     TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    {
        self.transform_inline(ri1, ri2, out, conv)
    }
}


impl<ITEMOUT> ZipSequenceTransform<ITEMOUT> for SelectedSyncAccumulationBorder<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    fn transform<'a, RI, ITEMIN, CTO>(
        &self,
        ri1:         RI,
        ri2:         RI,
        out:         &mut [ITEMOUT],
        _default_in: ITEMIN,
        conv:        CTO
    ) -> Result<usize, TransformResult>
    where
        RI:     TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    {
        self.transform_inline(ri1, ri2, out, conv)
    }
}

