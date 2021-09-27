//! # dxf2image
//!
//! `dxf2image` is a fast and efficient dxf to image converter!
//!
//! ## Usage
//!
//! ```
//! use dxf2image::{dxf2svg, dxf2png};
//!
//! fn main() {
//!     let dxf = "sample.dxf";
//!
//!     // Convert to svg
//!     let svg = "sample.svg";
//!     dxf2svg(dxf, svg).unwrap();
//!
//!     // Convert to png
//!     let png = "sample.png";
//!     dxf2png(dxf, png).unwrap();
//! }
//! ```

mod color;
mod coord;
mod dxf2svg;

pub use dxf2svg::dxf2svg;

#[cfg(feature = "png")]
pub use bitmap::svg2png;

#[cfg(feature = "png")]
/// Convert dxf to png
pub fn dxf2png(input_path: &str, output_path: &str) -> anyhow::Result<()> {
    // FIXME: use tmpfile
    dxf2svg(input_path, output_path)?;
    svg2png(output_path, output_path)?;
    Ok(())
}
