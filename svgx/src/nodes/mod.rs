mod circle;
mod ellipse;
mod line;
mod path;
mod polyline;
mod text;

pub trait Node {
    type Element: svg::Node;
    fn to_element(self) -> Self::Element;
}

macro_rules! define_node {
    ($node:ident, $svg_node:path, $new:expr) => {
        pub struct $node(pub (crate) $svg_node);

        impl Node for $node {
            type Element = $svg_node;

            fn to_element(self) -> Self::Element {
                self.0
            }
        }

        impl Default for $node {
            fn default() -> Self {
                Self($new)
                    .stroke_width(1.0)
                    .stroke("FFFFFF")
                    .fill("none")
            }
        }

        impl $node {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn stroke_width(self, value: f64) -> Self {
                Self(self.0.set("stroke-width", value))
            }

            pub fn stroke(self, value: &str) -> Self {
                Self(self.0.set("stroke", format!(r"#{}", value)))
            }

            pub fn fill(self, value: &str) -> Self {
                Self(self.0.set("fill", value))
            }
        }
    };
}

define_node!(Circle, svg::node::element::Circle, svg::node::element::Circle::new());
define_node!(Ellipse, svg::node::element::Ellipse, svg::node::element::Ellipse::new());
define_node!(Line, svg::node::element::Line, svg::node::element::Line::new());
define_node!(Path, svg::node::element::Path, svg::node::element::Path::new());
define_node!(Polyline, svg::node::element::Polyline, svg::node::element::Polyline::new());
define_node!(Text, svg::node::element::Text, svg::node::element::Text::new());
