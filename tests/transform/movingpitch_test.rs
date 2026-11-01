// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski


#[cfg(test)]
mod correctness_regressions {
    use sequencetransform::{ConvertToIdentity, TransformResult};
    use sequencetransform::transform::{MovingPitch, SequenceTransform};

    #[test]
    fn moving_pitch_skip_first_works_at_exact_window_size() {
        let data = [1.0_f32, 2.0, 3.0];
        let mut out = [0.0_f32; 3];
        let pitch = MovingPitch::<f32>::new(3, true);
        assert_eq!(pitch.transform(data.iter(), &mut out, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
        assert!((out[2] - 1.0).abs() < 0.0001);
    }

    #[test]
    fn moving_pitch_matches_known_linear_trend() {
        let data = [2.0_f32, 5.0, 8.0, 11.0, 14.0];
        let mut out = [0.0_f32; 5];
        let pitch = MovingPitch::<f32>::new(3, false);
        assert_eq!(pitch.transform(data.iter(), &mut out, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
        for value in out { assert!((value - 3.0).abs() < 0.0001); }
    }
}

#[cfg(test)]
mod invalid_window_regression {
    use sequencetransform::{ConvertToIdentity, TransformResult};
    use sequencetransform::transform::{MovingPitch, SequenceTransform};

    #[test]
    fn invalid_moving_window_returns_error() {
        let data = [1.0_f32, 2.0, 3.0];
        let mut out = [0.0_f32; 3];
        assert_ne!(MovingPitch::<f32>::new(1, true).transform(data.iter(), &mut out, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
    }
}
