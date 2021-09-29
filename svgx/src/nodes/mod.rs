mod circle;
mod line;
mod path;
mod polyline;

pub use circle::Circle;
pub use line::Line;
pub use path::Path;
pub use polyline::Polyline;

pub trait Node {
    type Element: svg::Node;
    fn to_element(self) -> Self::Element;
}
