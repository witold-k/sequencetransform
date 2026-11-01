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

#[cfg(test)]
mod numerical_stability_regressions {
    use sequencetransform::{ConvertToIdentity, TransformResult};
    use sequencetransform::transform::{MovingPitch, SequenceTransform};

    #[test]
    fn constant_large_offset_has_zero_slope() {
        let data = [1.0e12_f64; 64];
        let mut out = [f64::NAN; 64];
        let pitch = MovingPitch::<f64>::new(16, false);
        assert_eq!(pitch.transform(data.iter(), &mut out, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
        assert!(out.iter().all(|x| *x == 0.0));
    }

    #[test]
    fn small_slope_on_large_offset() {
        let data: Vec<f64> = (0..64).map(|i| 1.0e12 + i as f64 * 0.125).collect();
        let mut out = vec![0.0_f64; data.len()];
        let pitch = MovingPitch::<f64>::new(16, false);
        assert_eq!(pitch.transform(data.iter(), &mut out, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
        assert!(out.iter().all(|x| (*x - 0.125).abs() < 1.0e-9), "{out:?}");
    }
}
