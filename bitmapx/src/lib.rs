/// Convert svg to png
pub fn svg2png(input_path: &str, output_path: &str) -> anyhow::Result<()> {
    let mut opt = usvg::Options::default();
    // Get file's absolute directory.
    opt.resources_dir = std::fs::canonicalize(input_path)
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()));
    opt.fontdb.load_system_fonts();

    let svg_data = std::fs::read(input_path)?;
    let rtree = usvg::Tree::from_data(&svg_data, &opt.to_ref())?;

    let pixmap_size = rtree.svg_node().size.to_screen_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
        .ok_or(anyhow::anyhow!("`tiny_skia::Pixmap::new` is None"))?;
    resvg::render(&rtree, usvg::FitTo::Original, pixmap.as_mut())
        .ok_or(anyhow::anyhow!("`resvg::render` is None"))?;
    pixmap.save_png(output_path)?;
    Ok(())
}
