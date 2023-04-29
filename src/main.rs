extern crate rand;
extern crate rand_chacha;

use std::{ env, time::SystemTime };
use rand::{ Rng, SeedableRng };

fn main() {
	let args: Vec<String> = env::args().collect();
	// exit on poor argument parsing
	if args.len() == 1 || args.len() > 3 {
		panic!( "Usage: <matrix_dimension> [seed]" );
	}
	if *&args[1].parse::<usize>().unwrap() < 1 || *&args[1].parse::<usize>().unwrap() > 2000 {
		panic!( "{} is out of range.", *&args[1].parse::<usize>().unwrap() );
	}

	let mut seed: u64 = rand::random();
	let size: usize = *&args[1].parse::<usize>().unwrap();

	if args.len() == 3 {
		seed = *&args[2].parse::<u64>().unwrap();
		println!( "Seed = {}", seed );
	}
	
	println!( "Matrix dimension = {}", size );
	// because regular arrays can't have non constant size
	let mut a: Vec<f64> = vec![0.0; size * size];
	let mut b: Vec<f64> = vec![0.0; size];
	let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64( seed );

	
	for n in 0..( size * size ) {
		a[n] = rng.gen_range( 0.0..99999.99 );
	}

	for n in 0..size {
		b[n] = rng.gen_range( 0.0..99999.99 );
	}

	if size < 10 {
		println!( "A[" );
		for n in 0..( size * size ) {
			print!( "{var:>9.2} ", var = a[n] );
			if ( n % size ) == ( size - 1 ) {
				println!( "" );
			}
		}
		println!( "]" );
		println!( "" );
		print!( "B[" );
		for n in 0..size {
			print!( "{var:>9.2} ", var = b[n] );
		}
		print!( "]" );
		println!( "" );
	}

	gauss( a, b, size );
	
}

fn gauss( mut a: Vec<f64>, mut b: Vec<f64>, size: usize ) {
	let mut x: Vec<f64> = vec![0.0; size];
	let now = SystemTime::now();

	for norm in 0..( size - 1 ) {
		for row in ( norm + 1 )..size {
			let mult = a[( row * size ) + norm] / a[( norm * size ) + norm];
			for col in norm..size {
				a[( row * size ) + col] -= a[( norm * size ) + col] * mult;
			}
			b[row] -= b[norm] * mult;
		}
	}

	for row in ( 0..size ).rev() {
		x[row] = b[row];
		for col in ( ( row + 1 )..( size - 1 ) ).rev() {
			x[row] -= a[( row * size ) + col] * x[col];
		}
		x[row] /= a[( row * size ) + row];
	}

	match now.elapsed() {
		Ok( elapsed ) => {
			println!( "time: {}ms", elapsed.as_millis() );
		}
		Err( _e ) => {
			println!( "error" );
		}
	}

	if size < 100 {
		println!( "" );
		print!( "X[" );
		for n in 0..size {
			print!( "{var:>6.2} ", var = x[n] );
		}
		print!( "]" );
		println!( "" );
	}

}