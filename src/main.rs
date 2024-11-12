use rust_benchmarking::isr::inverse_square_root::InverseSquareRoot;

fn main() {
    println!("Hello, world!");
    
    let number: f32 = 12.24_f32;
    let result: f32 = number.isr();

    println!("Inverse Squre Root of {number} is {result}")
}
