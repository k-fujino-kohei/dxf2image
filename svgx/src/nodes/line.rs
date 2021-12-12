use super::Line;

impl Line {
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
