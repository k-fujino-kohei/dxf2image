use super::Polyline;

impl Polyline {
    pub fn points(self, value: Vec<(f64, f64)>) -> Self {
        Self(self.0.set("points", value))
    }
}
