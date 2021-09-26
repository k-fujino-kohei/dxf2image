mod color;
mod coord;
mod dxf2svg;

pub use dxf2svg::dxf2svg;

#[cfg(feature = "png")]
pub use bitmap::svg2png;

#[cfg(feature = "png")]
pub fn dxf2png(input_path: &str, output_path: &str) -> anyhow::Result<()> {
    // FIXME: use tmpfile
    dxf2svg(input_path, output_path)?;
    svg2png(output_path, output_path)?;
    Ok(())
}
