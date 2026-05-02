use rand::distr::Distribution;
use rand::distr::Uniform;
use std::env;
use std::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::rng();
    let gnrt = Uniform::new_inclusive(0, 15)?;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: cols not specified.");
        process::exit(1);
    }
    let cols = args[1].parse::<u32>();
    if let Err(_) = cols {
        eprintln!("Error: {} is invalid number.", args[1]);
        process::exit(1);
    }
    let cols = cols.unwrap();
    for _ in 0..cols {
        print!("{:x}", gnrt.sample(&mut rng));
    }
    println!();
    Ok(())
}
