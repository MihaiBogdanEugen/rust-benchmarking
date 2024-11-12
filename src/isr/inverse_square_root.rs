pub trait InverseSquareRoot {
    fn isr(&self) -> f32;
}

impl InverseSquareRoot for f32 {
    fn isr(&self) -> f32 {
        1_f32 / self.sqrt()
    }
}