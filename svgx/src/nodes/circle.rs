use super::Circle;

impl Circle {
    pub fn center(&self, center: (f64, f64)) -> Self {
        Self(self.0.to_owned().set("cx", center.0).set("cy", center.1))
    }

    pub fn radius(&self, radius: f64) -> Self {
        Self(self.0.to_owned().set("r", radius))
    }
}
