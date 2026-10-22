// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski


#[cfg(test)]
mod correctness_regressions {
    use sequencetransform::TransformResult;
    use sequencetransform::transform::{OneHotQuantization};

    #[test]
    fn one_hot_uses_distinct_chunks_and_clamps_upper_edge() {
        let data = [0.0_f32, 0.5, 1.0];
        let mut out = [9.0_f32; 12];
        let trans = OneHotQuantization::<f32>::new(4);
        assert_eq!(trans.transform_self(data.iter(), &mut out), TransformResult::Ok);
        assert_eq!(out, [1.,0.,0.,0., 0.,0.,1.,0., 0.,0.,0.,1.]);
    }
}
