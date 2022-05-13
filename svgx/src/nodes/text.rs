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

    pub fn text_length(&self, length: f64) -> Self {
        Self(
            self.0.to_owned()
                .set("textLength", length)
        )
    }

    pub fn text_anchor(&self, anchor: TextAnchor) -> Self {
        let anchor = match anchor {
            TextAnchor::Start => "start",
            TextAnchor::Middle => "middle",
            TextAnchor::End => "end",
        };
        Self(
            self.0.to_owned()
                .set("text-anchor", anchor)
        )
    }

    pub fn text_decoration(&self, value: &str) -> Self {
        Self(
            self.0.to_owned()
                .set("text-decoration", value)
        )
    }
}

/// https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/text-anchor
pub enum TextAnchor {
    Start,
    Middle,
    End
}
