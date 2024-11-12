pub trait InverseSquareRoot {
    fn isr(&self) -> f32;

    fn isr_optimized_unsafe(&self) -> f32;

    fn isr_optimized(&self) -> f32;
}

impl InverseSquareRoot for f32 {
    fn isr(&self) -> f32 {
        1_f32 / self.sqrt()
    }

    #[allow(clippy::transmute_float_to_int)]
    #[allow(clippy::transmute_int_to_float)]
    fn isr_optimized_unsafe(&self) -> f32 {
        let mut y: f32 = *self;
        unsafe {
            let mut i: i32 = std::mem::transmute::<f32, i32>(y);
            i = 0x5F375A86 - (i >> 1);
            y = std::mem::transmute::<i32, f32>(i);
        }
        y * (1.5 - (self * 0.5 * y * y))
    }

    fn isr_optimized(&self) -> f32 {
        let mut i: i32 = self.to_bits() as i32;
        i = 0x5F375A86_i32.wrapping_sub(i >> 1);
        let y: f32 = f32::from_bits(i as u32);
        y * (1.5 - (self * 0.5 * y * y))
    }
}
