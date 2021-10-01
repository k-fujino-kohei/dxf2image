use std::f64::consts::PI;
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

    pub fn arc(
        center: (f64, f64),
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        is_clockwise: bool,
    ) -> Self {
        fn polar_to_cartesian(
            center: (f64, f64),
            radius: f64,
            angle_in_degrees: f64,
        ) -> (f64, f64) {
            let angle_in_radians = (angle_in_degrees - 180.0) * (PI / 180.0);
            (
                center.0 + (radius * angle_in_radians.cos()),
                center.1 + (radius * angle_in_radians.sin()),
            )
        }
        let (x1, y1) = polar_to_cartesian(center, radius, end_angle);
        let (x2, y2) = polar_to_cartesian(center, radius, start_angle);
        let rx = radius;
        let ry = radius;
        let large_arc_flag = if (end_angle - start_angle) <= 180.0 {
            0
        } else {
            1
        };
        let sweep_flag = if is_clockwise { 1 } else { 0 };
        let d = Data::new().move_to((x1, y1)).elliptical_arc_to((
            rx,
            ry,
            0,
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
