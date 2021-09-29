use svg::node::element::path::Data;

use super::Node;

pub struct Path(svg::node::element::Path);

impl Node for Path {
    type Element = svg::node::element::Path;

    fn to_element(self) -> Self::Element {
        self.0
    }
}

impl Path {
    pub fn new() -> Self {
        Self(svg::node::element::Path::new())
            .stroke_width(1)
            .stroke("#FFFFFF")
            .fill("none")
    }

    pub fn arc(center: (f64, f64), radius: f64, start_angle: f64, end_angle: f64) -> Self {
        let x1 = radius * start_angle.cos() + center.0;
        let y1 = radius * start_angle.sin() + center.1;
        let rx = radius;
        let ry = radius;
        let large_arc_flag = if (start_angle - end_angle).abs() < 180.0 {
            0
        } else {
            1
        };
        let sweep_flag = if start_angle > end_angle { 0 } else { 1 };
        let x2 = radius * end_angle.cos() + center.0;
        let y2 = radius * end_angle.sin() + center.1;
        let d = Data::new().move_to((x1, y1)).elliptical_arc_to((
            rx,
            ry,
            start_angle,
            large_arc_flag,
            sweep_flag,
            x2,
            y2,
        ));
        Self::new().set(d)
    }

    fn set<U>(self, data: U) -> Self
    where
        U: Into<svg::node::Value>,
    {
        Self(self.0.set("d", data))
    }
}

impl Path {
    pub fn stroke_width(self, value: usize) -> Self {
        Self(self.0.set("stroke-width", value))
    }

    pub fn stroke(self, value: &str) -> Self {
        Self(self.0.set("stroke", format!(r"#{}", value)))
    }

    pub fn fill(self, value: &str) -> Self {
        Self(self.0.set("fill", value))
    }
}
