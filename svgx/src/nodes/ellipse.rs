use super::Ellipse;

impl Ellipse {
    pub fn center(&self, center: (f64, f64)) -> Self {
        Self(self.0.to_owned().set("cx", center.0).set("cy", center.1))
    }

    pub fn radius(&self, radius: (f64, f64)) -> Self {
        Self(self.0.to_owned().set("rx", radius.0).set("ry", radius.1))
    }
}
