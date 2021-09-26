use super::Node;

pub struct Line(svg::node::element::Line);

impl Node for Line {
    type Element = svg::node::element::Line;

    fn to_element(self) -> Self::Element {
        self.0
    }
}

impl Line {
    pub fn new() -> Self {
        Self(svg::node::element::Line::new())
            .stroke_width(1)
            .stroke("000000")
    }

    pub fn points(&self, from: (f64, f64), to: (f64, f64)) -> Self {
        Self(
            self.0
                .to_owned()
                .set("x1", from.0)
                .set("y1", from.1)
                .set("x2", to.0)
                .set("y2", to.1),
        )
    }
}

// TODO: DRY
impl Line {
    pub fn stroke_width(self, value: usize) -> Self {
        Self(self.0.set("stroke-width", value))
    }

    pub fn stroke(self, value: &str) -> Self {
        Self(self.0.set("stroke", format!(r"#{}", value)))
    }
}
