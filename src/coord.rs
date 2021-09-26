pub struct Coord {
    extmax_x: f64,
    extmax_y: f64,
    extmin_x: f64,
    extmin_y: f64,
    zoom_scale: f64,
}

impl Coord {
    pub fn new(extmax: dxf::Point, extmin: dxf::Point) -> Self {
        Self {
            extmax_x: extmax.x,
            extmax_y: extmax.y,
            extmin_x: extmin.x,
            extmin_y: extmin.y,
            // FIXME: calculate the scale
            zoom_scale: 10.0,
        }
    }

    pub fn origin(&self) -> (f64, f64) {
        (0.0, 0.0)
    }

    pub fn base_point(&self) -> (f64, f64) {
        (self.extmax_x, self.extmax_y)
    }

    pub fn width(&self) -> f64 {
        (self.extmax_x - self.extmin_x).abs() / self.zoom_scale
    }

    pub fn height(&self) -> f64 {
        (self.extmax_y - self.extmin_y).abs() / self.zoom_scale
    }
}

pub trait PointConverter<P> {
    type Output;
    fn relative_to(&self, point: P) -> Self::Output;
}

impl PointConverter<(f64, f64)> for Coord {
    type Output = (f64, f64);
    fn relative_to(&self, point: (f64, f64)) -> (f64, f64) {
        (
            (self.base_point().0 - point.0) / self.zoom_scale,
            (self.base_point().1 - point.1) / self.zoom_scale,
        )
    }
}

impl PointConverter<dxf::Point> for Coord {
    type Output = (f64, f64);
    fn relative_to(&self, point: dxf::Point) -> (f64, f64) {
        self.relative_to((point.x, point.y))
    }
}

impl PointConverter<f64> for Coord {
    type Output = f64;
    fn relative_to(&self, length: f64) -> f64 {
        length / self.zoom_scale
    }
}
