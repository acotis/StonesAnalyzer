
use std::f32::consts::TAU;

pub type Layout = Vec::<(f32, f32)>;

pub trait LayoutTrait {
    fn transform(self, x0: f32, x1: f32, y0: f32, y1: f32, s0: f32, s1: f32) -> Layout;
    fn scale(self, factor: f32) -> Layout;
    fn shift(self, dx: f32, dy: f32) -> Layout;
    fn rotate(self, fraction: f32) -> Layout;
    fn mirror(self) -> Layout;
}

impl LayoutTrait for Layout {
    // Transform a layout by the augmented matrix:
    //     [x0 y0 | s0]
    //     [x1 y1 | s1]

    fn transform(self, x0: f32, x1: f32, y0: f32, y1: f32, s0: f32, s1: f32) -> Layout {
        self.into_iter()
            .map(|p| (p.0 * x0 + p.1 * y0 + s0, p.0 * x1 + p.1 * y1 + s1))
            .collect()
    }

    fn scale(self, factor: f32) -> Layout {
        self.transform(factor, 0.0, 0.0, factor, 0.0, 0.0)
    }

    fn shift(self, dx: f32, dy: f32) -> Layout {
        self.transform(1.0, 0.0, 0.0, 1.0, dx, dy)
    }

    fn mirror(self) -> Layout {
        self.transform(-1.0, 0.0, 0.0, 1.0, 0.0, 0.0)
    }

    fn rotate(self, fraction: f32) -> Layout {
        let angle = fraction * TAU;
        self.transform(angle.cos(), -angle.sin(), angle.sin(), angle.cos(), 0.0, 0.0)
    }
}

