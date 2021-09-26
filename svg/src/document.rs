use crate::nodes::Node;

pub struct Document(svg::Document);

impl Document {
    pub fn new() -> Self {
        Self(svg::Document::new())
    }

    pub fn viewbox(self, x: f64, y: f64, width: f64, height: f64) -> Self {
        let doc = self
            .0
            .set("width", width)
            .set("height", height)
            .set("viewbox", (x, y, width, height));
        Self(doc)
    }

    pub fn add(&mut self, node: impl Node) {
        self.0 = self.0.to_owned().add(node.to_element());
    }

    pub fn save(self, path: &str) -> anyhow::Result<()> {
        svg::save(path, &self.0).map_err(|e| e.into())
    }
}
