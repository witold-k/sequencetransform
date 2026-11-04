// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski
//! Decode all fifty plotted SVG output points and check against Rust transforms.
use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::transform::*;
use sequencetransform::transform::sort::Sort;
use sequencetransform::ziptransform::*;
use std::fs;

const NAMES: [&str; 38] = ["deltasignum","deltasignumsum","differentiate","heavyside","integrate","maxdirection","minusdistance","minusdistancesum","movingaverage","movingaverageaverage","movingdirection","movingmaxcrossinglevel","movingmeansquareerror","movingmedian","movingmedian2","movingmedianaverage","movingmedianaverage2","movingpitch","movingautoregression","movingautoregressionchange","movingexplainedvariance","movingpermutationentropy","movingtimereversalasymmetry","movingspectralentropy","movingspectralcentroid","movingspectralchange","movingpositivestrength","movingrootmeansquareerror","movingsquaredweightedmedianposition","movingweightedmedianposition","onehotquantization","order","relativestrength","relativestrengthweigth","relativestrengthweigthsum","signum","signumsum","sort"];

fn array(svg: &str, key: &str) -> Vec<f64> {
    let needle = format!("\"{key}\":[");
    let start = svg.find(&needle).expect("metadata array") + needle.len();
    let end = start + svg[start..].find(']').expect("array end");
    svg[start..end].split(',').map(|s| s.parse().expect("f64")).collect()
}
fn run<T: SequenceTransform<f64>>(t: T, input: &[f64]) -> Vec<f64> {
    let mut out = vec![0.0; input.len()];
    assert_eq!(t.transform(input.iter(), &mut out, 0.0, ConvertToIdentity::<f64>::default()), TransformResult::Ok);
    out
}
fn calculate(name: &str, input: &[f64]) -> Vec<f64> {
    match name {
        "deltasignum" => run(DeltaSignum::<f64>::default(), input),
"deltasignumsum" => run(DeltaSignumSum::<f64>::default(), input),
"differentiate" => run(Differentiate::<f64>::default(), input),
"heavyside" => run(Heavyside::<f64>::default(), input),
"integrate" => run(Integrate::<f64>::default(), input),
"maxdirection" => run(MaxDirection::<f64>::new(9), input),
"minusdistance" => run(MinusDistance::<f64>::default(), input),
"minusdistancesum" => run(MinusDistanceSum::<f64>::default(), input),
"movingaverage" => run(MovingAverage::<f64>::new(9), input),
"movingaverageaverage" => run(MovingAverageAverage::<f64>::new(7,5), input),
"movingdirection" => run(MovingDirection::<f64>::new(10), input),
"movingmaxcrossinglevel" => run(MovingMaxCrossingLevel::<f64>::new(10), input),
"movingmedian" => run(MovingMedian::<f64>::new(9), input),
"movingmedian2" => run(MovingMedian2::<f64>::new(8), input),
"movingmedianaverage" => run(MovingMedianAverage::<f64>::new(9,5), input),
"movingmedianaverage2" => run(MovingMedianAverage2::<f64>::new(8,5), input),
"movingpitch" => run(MovingPitch::<f64>::new(9,false), input),
"movingautoregression" => run(MovingAutoregression::<f64>::new(9), input),
"movingautoregressionchange" => run(MovingAutoregressionChange::<f64>::new(12), input),
"movingexplainedvariance" => run(MovingExplainedVariance::<f64>::new(9), input),
"movingpermutationentropy" => run(MovingPermutationEntropy::<f64>::new(12,3), input),
"movingtimereversalasymmetry" => run(MovingTimeReversalAsymmetry::<f64>::new(12), input),
"movingspectralentropy" => run(MovingSpectralEntropy::<f64>::new(12), input),
"movingspectralcentroid" => run(MovingSpectralCentroid::<f64>::new(12), input),
"movingspectralchange" => run(MovingSpectralChange::<f64>::new(12), input),
"movingpositivestrength" => run(MovingPositiveStrength::<f64>::new(10), input),
"movingsquaredweightedmedianposition" => run(MovingSquaredWeightedMedianPosition::<f64>::new(9), input),
"movingweightedmedianposition" => run(MovingWeightedMedianPosition::<f64>::new(9), input),
"order" => run(Order::<f64>{factor:0.0}, input),
"relativestrength" => run(RelativeStrength::<f64>::new(10,false), input),
"relativestrengthweigth" => run(RelativeStrengthWeigth::<f64>::new(10,false), input),
"relativestrengthweigthsum" => run(RelativeStrengthWeigthSum::<f64>::new(10,false), input),
"signum" => run(Signum::<f64>::default(), input),
"signumsum" => run(SignumSum::<f64>::default(), input),
"sort" => run(Sort::<f64>::new(), input),
        "movingmeansquareerror" | "movingrootmeansquareerror" => {
            let a = vec![0.0; input.len()];
            let b: Vec<f64> = input.iter().map(|v| v.sqrt()).collect();
            let mut out = vec![0.0; input.len()];
            let result = if name == "movingmeansquareerror" {
                MovingMeanSquareError::<f64>::new(9).transform(a.iter(), b.iter(), &mut out, 0.0, ConvertToIdentity::<f64>::default())
            } else {
                MovingRootMeanSquareError::<f64>::new(9).transform(a.iter(), b.iter(), &mut out, 0.0, ConvertToIdentity::<f64>::default())
            };
            assert_eq!(result, Ok(input.len()));
            out
        },
        "onehotquantization" => {
            let mut out = vec![0.0; input.len() * 8];
            assert_eq!(OneHotQuantization::<f64>::new(8).transform(input.iter(), &mut out, 0.0, ConvertToIdentity::<f64>::default()), TransformResult::Ok);
            out.chunks_exact(8).map(|c| c.iter().position(|v| *v == 1.0).unwrap() as f64).collect()
        },
        _ => panic!("missing {name}"),
    }
}
fn svg_curve(svg: &str, lo: f64, hi: f64, color: &str) -> Vec<f64> {
    let marker = format!("fill=\"none\" stroke=\"{color}\"");
    let end = svg.find(&marker).expect("plotted curve");
    let prefix = &svg[..end];
    let start = prefix.rfind("<path d=\"").expect("path") + "<path d=\"".len();
    let path = prefix[start..].split('"').next().unwrap();
    path.split_whitespace().map(|p| {
        let (_, y) = p[1..].split_once(',').expect("SVG point");
        let pixel_y: f64 = y.parse().unwrap();
        lo + (218.0 - pixel_y) * (hi - lo) / 165.0
    }).collect()
}
#[test]
fn svg_effect_curves_match_actual_rust_transforms() {
    for name in NAMES {
        let path = format!("{}/docs/transform/assets/{name}-effect.svg", env!("CARGO_MANIFEST_DIR"));
        let svg = fs::read_to_string(path).expect("SVG exists");
        assert!(svg.contains("data-sample-count=\"50\""), "{name}");
        let input = array(&svg, "input");
        let stored = array(&svg, "output");
        let input_scale = array(&svg, "input_range");
        let output_scale = array(&svg, "output_range");
        assert_eq!(input.len(), 50, "{name}");
        assert_eq!(stored.len(), 50, "{name}");
        assert_eq!(input_scale.len(), 2, "{name}");
        assert_eq!(output_scale.len(), 2, "{name}");
        assert!(svg.contains("Input scale (left)"), "{name}");
        assert!(svg.contains("Output scale (right)"), "{name}");
        let actual = calculate(name, &input);
        let decoded = svg_curve(&svg, output_scale[0], output_scale[1], "#c45126");
        let decoded_input = svg_curve(&svg, input_scale[0], input_scale[1], "#1672ac");
        assert_eq!(decoded.len(), 50, "{name}");
        assert_eq!(decoded_input.len(), 50, "{name}");
        let input_tol = (input_scale[1]-input_scale[0]).abs() * 0.00051 / 165.0 + 1e-8;
        for i in 0..50 {
            assert!((actual[i]-stored[i]).abs() < 1e-7 * actual[i].abs().max(1.0),
                "{name}[{i}]: Rust={} metadata={}", actual[i], stored[i]);
            assert!((input[i]-decoded_input[i]).abs() < input_tol,
                "{name}[{i}]: input={} decoded input pixels={}", input[i], decoded_input[i]);
            let tol = (output_scale[1]-output_scale[0]).abs() * 0.00051 / 165.0 + 1e-8;
            assert!((actual[i]-decoded[i]).abs() < tol,
                "{name}[{i}]: Rust={} decoded pixels={}", actual[i], decoded[i]);
        }
    }
}
