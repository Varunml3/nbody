use std::{cmp::min};



fn main() {
    let mut threads = std::env::var("THREADS").map(|e| e.parse::<i32>().expect("Has to be a number")).unwrap_or(num_cpus::get() as i32);
    println!("cargo:rustc-env=THREADS={}", threads);

    let mut particles = std::env::var("PARTICLES").map(|e| e.parse::<i32>().expect("Invalid number of particles")).unwrap_or(100);

    threads *= 8;

    particles += min((particles % threads).abs(), (threads - (particles % threads)).abs());

    assert!(particles % threads == 0);
    println!("cargo:rustc-env=PARTICLES={}", particles);
}