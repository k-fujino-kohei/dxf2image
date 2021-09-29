use super::Node;

pub struct Circle(svg::node::element::Circle);

impl Node for Circle {
    type Element = svg::node::element::Circle;

    fn to_element(self) -> Self::Element {
        self.0
    }
}

impl Circle {
    pub fn new() -> Self {
        Self(svg::node::element::Circle::new())
            .stroke("000000")
            .fill("none")
    }

    pub fn center(&self, center: (f64, f64)) -> Self {
        Self(self.0.to_owned().set("cx", center.0).set("cy", center.1))
    }

    pub fn radius(&self, radius: f64) -> Self {
        Self(self.0.to_owned().set("r", radius))
    }
}

impl Circle {
    pub fn stroke(self, value: &str) -> Self {
        Self(self.0.set("stroke", format!(r"#{}", value)))
    }

    pub fn fill(self, value: &str) -> Self {
        Self(self.0.set("fill", value))
    }
}
