extern crate rand;
extern crate rand_chacha;

use std::{ env, time::SystemTime };
use rand::{ Rng, SeedableRng };
use rand_chacha::ChaCha8Rng;

const MAXN: usize = 2000;

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut seed: u64 = rand::random();
	let n: usize = parameters( args, &mut seed );
	let rng = rand_chacha::ChaCha8Rng::seed_from_u64( seed );
	let mut a: Vec<f64> = vec![0.0; n * n];
	let mut b: Vec<f64> = vec![0.0; n];
	let mut x: Vec<f64> = vec![0.0; n];
	initialize( n, rng, a.as_mut_slice(), b.as_mut_slice() );
	printing( n, a.as_mut_slice(), b.as_mut_slice() );
	
	println!( "Starting clock." );
	let now = SystemTime::now();

	gauss( n, a.as_mut_slice(), b.as_mut_slice(), x.as_mut_slice() );

	println!( "Stopped clock." );
	match now.elapsed() {
		Ok( elapsed ) => {
			printing_x( n, x.as_mut_slice() );
			println!( "Time elapsed: {}ms", elapsed.as_millis() );
		}
		Err( _e ) => {
			println!( "ERROR" );
		}
	}
}

fn parameters( args: Vec<String>, seed: &mut u64 ) -> usize {
	let n: usize;
	if args.len() == 3 {
		*seed = args[2].parse::<u64>().unwrap();
		println!( "Seed = {}", seed );
	}
	if args.len() >= 2 {
		n = *&args[1].parse::<usize>().unwrap();
		if n < 1 || n > MAXN {
			panic!( "{} is out of range.", n );
		}
	} else { // exit on poor argument parsing
		panic!( "Usage: <matrix_dimension> [seed]" );
	}

	println!( "Matrix dimension = {}", n );
	return n;
}

fn initialize( n: usize, mut rng: ChaCha8Rng, a: &mut [f64], b: &mut [f64] ) {
	println!( "Initializing. . ." );
	for i in 0..( n * n ) {
		a[i] = rng.gen_range( 0.01..99999.99 );
	}

	for i in 0..n {
		b[i] = rng.gen_range( 0.01..99999.99 );
	}
}

fn printing( n: usize, a: &mut [f64], b: &mut [f64] ) {
	if n < 10 {
		println!( "A[ " );
		for row in 0..n {
			for col in 0..n {
				print!( "{var:>9.2} ", var = a[( row * n ) + col] );
			}
			println!("");
		}
		print!( "{}", format!( "] \n\nB[ ") );
		for i in 0..n {
			print!( "{var:>9.2} ", var = b[i] );
		}
		println!( "{}", format!( "] \n") );
	}
}

fn printing_x( n: usize, x: &mut [f64] ) {
	if n < 100 {
		print!( "{}", format!( "\nX[ ") );
		for i in 0..n {
			print!( "{var:>6.2} ", var = x[i] );
		}
		println!( "{}", format!( "] \n") );
	}
}

fn gauss( n: usize, a: &mut [f64], b: &mut [f64], x: &mut [f64] ) {
	println!( "Computing. . ." );
	for norm in 0..( n - 1 ) {
		for row in ( norm + 1 )..n {
			let mult = a[( row * n ) + norm] / a[( norm * n ) + norm];
			for col in norm..n {
				a[( row * n ) + col] -= a[( norm * n ) + col] * mult;
			}
			b[row] -= b[norm] * mult;
		}
	}

	for row in ( 0..n ).rev() {
		x[row] = b[row];
		for col in ( ( row + 1 )..( n - 1 ) ).rev() {
			x[row] -= a[( row * n ) + col] * x[col];
		}
		x[row] /= a[( row * n ) + row];
	}
}