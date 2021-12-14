#[derive(Debug, Clone)]
pub struct Coord {
    extmax_x: f64,
    extmax_y: f64,
    extmin_x: f64,
    extmin_y: f64,
    scale_factor: f64,
    base_point: (f64, f64),
    original_width: f64,
    original_height: f64,
    /// origin of all drawing extents
    origin: (f64, f64),
    position: Position,
}

#[derive(Debug, Clone)]
pub enum Position {
    Absolute, Relative
}

impl Coord {
    pub fn new(extmax: dxf::Point, extmin: dxf::Point, max_length: Option<f64>) -> Self {
        let width = (extmax.x - extmin.x).powi(2).sqrt();
        let height = (extmax.y - extmin.y).powi(2).sqrt();
        let scale_factor = max_length
            .map(|max_length| (width.max(height), max_length))
            .filter(|(long_length, max_length)| long_length > max_length)
            .map(|(long_length, max_length)| long_length / max_length)
            .unwrap_or(1.0);

        Self {
            extmax_x: extmax.x,
            extmax_y: extmax.y,
            extmin_x: extmin.x,
            extmin_y: extmin.y,
            scale_factor,
            base_point: (extmin.x, extmax.y),
            original_width: width,
            original_height: height,
            origin: (extmin.x, extmax.y),
            position: Position::Absolute,
        }
    }

    pub fn set_base_point(&mut self, base_point: (f64, f64), position: Position) {
        self.base_point = (
            (self.origin.0 - base_point.0).abs(),
            (self.origin.1 - base_point.1).abs(),
        );
        self.position = position;
    }

    pub fn base_point(&self) -> (f64, f64) {
        self.base_point
    }

    pub fn width(&self) -> f64 {
        self.original_width / self.scale_factor
    }

    pub fn height(&self) -> f64 {
        self.original_height / self.scale_factor
    }
}

pub trait PointConverter<P> {
    type Output;
    fn relative_to(&self, point: P) -> Self::Output;
}

impl PointConverter<(f64, f64)> for Coord {
    type Output = (f64, f64);
    fn relative_to(&self, point: (f64, f64)) -> (f64, f64) {
        let (x, y) = (point.0, point.1);
        match self.position {
            Position::Absolute => {
                (
                    (x - self.base_point().0).abs() / self.scale_factor,
                    (y - self.base_point().1).abs() / self.scale_factor,
                )
            },
            Position::Relative => {
                (
                    (self.base_point().0 + x) / self.scale_factor,
                    (self.base_point().1 - y) / self.scale_factor,
                )
            }
        }
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
        length / self.scale_factor
    }
}
