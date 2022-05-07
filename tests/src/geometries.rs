//! Tests for the data files from https://github.com/horizon-eda/horizon/
//!
//! Horizon is an app Electronic Design Automation.  It has SVG templates with specially
//! named elements; the app extracts their geometries and renders GUI widgets instead of
//! those elements.  So, it is critical that the geometries get computed accurately.
//!
//! Horizon's build system pre-computes the geometries of the SVG templates' elements, and
//! stores them in JSON files.  You can see the SVGs and the .subs JSON files in the
//! tests/fixtures/horizon in the librsvg source tree.
//!
//! This test file has machinery to load the SVG templates, and the JSON files with the
//! expected geometries.  The tests check that librsvg computes the same geometries every
//! time.

use anyhow::{Context, Result};
use librsvg::{CairoRenderer, LengthUnit, Loader};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

// Copy of cairo::Rectangle
//
// Somehow I can't make serde's "remote" work here, in combination with the BTreeMap below...
#[derive(Copy, Clone, Deserialize, Debug, PartialEq)]
struct Rectangle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl From<Rectangle> for cairo::Rectangle {
    fn from(r: Rectangle) -> cairo::Rectangle {
        cairo::Rectangle {
            x: r.x,
            y: r.y,
            width: r.width,
            height: r.height,
        }
    }
}

#[derive(Deserialize)]
struct Geometries(BTreeMap<String, Rectangle>);

fn read_geometries(path: &Path) -> Result<Geometries> {
    let contents = fs::read_to_string(path).context(format!("could not read {:?}", path))?;
    Ok(serde_json::from_str(&contents).context(format!("could not parse JSON from {:?}", path))?)
}

// We create a struct with the id and geometry so that
// assert_eq!() in the tests will print out the element name for failures.
#[derive(Debug, PartialEq)]
struct Element {
    id: String,
    geom: cairo::Rectangle,
}

fn test(svg_filename: &str) {
    let mut geometries_filename = String::from(svg_filename);
    geometries_filename.push_str(".subs");

    let geometries =
        read_geometries(Path::new(&geometries_filename)).expect("reading geometries JSON");

    let handle = Loader::new()
        .read_path(svg_filename)
        .expect("reading geometries SVG");
    let renderer = CairoRenderer::new(&handle);
    let dimensions = renderer.intrinsic_dimensions();
    let (svg_width, svg_height) = renderer
        .intrinsic_size_in_pixels()
        .expect("intrinsic size in pixels");

    assert!(matches!(dimensions.width.unit, LengthUnit::Px));
    assert!(matches!(dimensions.height.unit, LengthUnit::Px));
    assert_eq!(dimensions.width.length, svg_width);
    assert_eq!(dimensions.height.length, svg_height);

    for (id, expected) in geometries.0.iter() {
        println!("id: {}", id);
        let expected = Element {
            id: String::from(id),
            geom: cairo::Rectangle::from(*expected),
        };

        let viewport = cairo::Rectangle {
            x: 0.0,
            y: 0.0,
            width: svg_width,
            height: svg_height,
        };

        let (geometry, _) = renderer
            .geometry_for_layer(Some(id), &viewport)
            .expect(&format!("getting geometry for {}", id));

        let computed = Element {
            id: String::from(id),
            geom: geometry,
        };

        assert_eq!(expected, computed);
    }
}

#[test]
fn dual() {
    test("tests/fixtures/geometries/dual.svg");
}

#[test]
fn grid() {
    test("tests/fixtures/geometries/grid.svg");
}

#[test]
fn quad() {
    test("tests/fixtures/geometries/quad.svg");
}

#[test]
fn single() {
    test("tests/fixtures/geometries/single.svg");
}
