// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::fs;
use std::path::PathBuf;

#[test]
fn all_transform_pages_have_english_text_and_both_svg_visualizations() {
    let root=PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs/transform");
    let names=[
        "deltasignum","deltasignumsum","differentiate","heavyside","integrate",
        "maxdirection","minusdistance","minusdistancesum","movingaverage",
        "movingaverageaverage","movingmedian","movingmedian2","movingmedianaverage",
        "movingmedianaverage2","movingpitch","movingautoregression","movingautoregressionchange","movingexplainedvariance","movingpermutationentropy","movingtimereversalasymmetry","onehotquantization","order",
        "relativestrength","relativestrengthweigth","relativestrengthweigthsum",
        "signum","signumsum","sort","movingweightedmedianposition",
        "movingsquaredweightedmedianposition","movingpositivestrength",
        "movingdirection","movingmaxcrossinglevel","movingmeansquareerror",
        "movingrootmeansquareerror",
    ];
    let index=fs::read_to_string(root.join("README.md")).unwrap();
    let main=fs::read_to_string(root.join("../../README.md")).unwrap();
    assert!(main.contains("docs/transform/README.md"));
    assert!(index.contains("## Architecture"));
    assert!(index.contains("## Notes"));
    assert!(!index.contains("Haupt-README") && !index.contains("Einzelne Transforms") && !index.contains("Hinweise"));
    for name in names {
        let page=fs::read_to_string(root.join(format!("{name}.md"))).unwrap();
        assert!(index.contains(&format!("{name}.md")), "index missing {name}");
        assert!(page.contains("## Algorithm"), "algorithm missing {name}");
        assert!(page.contains("## Input versus output"), "effect missing {name}");
        assert!(page.contains("## Window diagram"), "window missing {name}");
        assert!(page.contains("Rust source"), "source missing {name}");
        for kind in ["window","effect"] {
            let asset=format!("assets/{name}-{kind}.svg");
            assert!(page.contains(&asset), "missing link to {asset}");
            let svg=fs::read_to_string(root.join(&asset)).unwrap();
            assert!(svg.contains("<svg") && svg.contains("</svg>"), "invalid SVG {asset}");
            assert!(svg.contains("role=\"img\""), "missing accessibility metadata {asset}");
            assert!(svg.contains("viewBox="), "missing viewBox {asset}");
            assert!(svg.contains(name), "wrong algorithm label in {asset}");
            assert!(!svg.contains("Zeitreihe") && !svg.contains("aktuelles Fenster"), "untranslated SVG {asset}");
        }
    }
}
