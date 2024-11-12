# rust-benchmarking
Playing around with benchmarking options in Rust

## Requirements
- Rust nightly
```shell
$ rustc --version
rustc 1.84.0-nightly (81eef2d36 2024-11-11)
```
- `rustfmt` and `clippy` for code quality

## Run
```shell
$ make run val=0.123456
```

## Benchmark
```shell
$ make bench
...
running 3 tests
test tests::bench_isr                  ... bench:          99.05 ns/iter (+/- 4.64)
test tests::bench_isr_optimized        ... bench:          35.96 ns/iter (+/- 1.38)
test tests::bench_isr_optimized_unsafe ... bench:          36.58 ns/iter (+/- 2.59)
```

## Fast Inverse Square Root
```c
float Q_rsqrt( float number )
{
	long i;
	float x2, y;
	const float threehalfs = 1.5F;

	x2 = number * 0.5F;
	y  = number;
	i  = * ( long * ) &y;						// evil floating point bit level hacking
	i  = 0x5f3759df - ( i >> 1 );               // what the fuck?
	y  = * ( float * ) &i;
	y  = y * ( threehalfs - ( x2 * y * y ) );   // 1st iteration
//	y  = y * ( threehalfs - ( x2 * y * y ) );   // 2nd iteration, this can be removed

	return y;
}
```

More details [here](https://www.youtube.com/watch?v=p8u_k2LIZyo).
 