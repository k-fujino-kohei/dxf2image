use super::Node;

pub struct Polyline(svg::node::element::Polyline);

impl Node for Polyline {
    type Element = svg::node::element::Polyline;

    fn to_element(self) -> Self::Element {
        self.0
    }
}

impl Polyline {
    pub fn new() -> Self {
        Self(svg::node::element::Polyline::new())
            .stroke_width(1)
            .stroke("000000")
    }

    pub fn points(self, value: Vec<(f64, f64)>) -> Self {
        Self(self.0.set("points", value))
    }
}

impl Polyline {
    pub fn stroke_width(self, value: usize) -> Self {
        Self(self.0.set("stroke-width", value))
    }

    pub fn stroke(self, value: &str) -> Self {
        Self(self.0.set("stroke", format!(r"#{}", value)))
    }
}
