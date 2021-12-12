use super::Text;

impl Text {
    pub fn content(&self, content: &str) -> Self {
        Self(
            self.0.to_owned()
                .add(svg::node::Text::new(content))
        )
    }

    pub fn points(&self, point: (f64, f64)) -> Self {
        Self(
            self.0
                .to_owned()
                .set("x", point.0)
                .set("y", point.1)
        )
    }

    pub fn font_size(&self, size: f64) -> Self {
        Self(
            self.0.to_owned()
                .set("font-size", size)
        )
    }
}
