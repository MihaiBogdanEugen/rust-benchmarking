use clap::Parser;
use rust_benchmarking::isr::inverse_square_root::InverseSquareRoot;

#[derive(Parser)]
#[command()]
struct Cli {
    #[arg(short, long, value_parser = clap::value_parser!(f32))]
    val: f32,
}

fn main() {
    let cli: Cli = Cli::parse();
    let number: f32 = cli.val;
    let result: f32 = number.isr();

    println!("Inverse Squre Root of {number} is {result}")
}
