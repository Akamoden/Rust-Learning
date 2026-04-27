// todo : create structs for 2d vectors, and find the sum of them. 

use rand::Rng;
use rand::RngExt; // hack could use prelude but this is more explicit.
#[derive(Debug)] 
struct Vector2 {
    x: f64,
    y: f64,
}

impl Vector2 {
    fn new(x: f64, y: f64) -> Self {
        Vector2 { x, y }
    }
    
    fn random(rng: &mut impl Rng) -> Self { // had trouble with this, had to look up how to use the rng in a function.
        Self::new(
            rng.random_range(-100.0..100.0),
            rng.random_range(-100.0..100.0),
        )
    }
    
    fn add(&self, other: &Vector2) -> Vector2 {
        Self::new(self.x + other.x, self.y + other.y) // insight : had to watch a video on how to find the sum of two vectors.
    }
}

fn main() {
    let mut rng = rand::rng();
    
    let a = Vector2::random(&mut rng);
    let b = Vector2::random(&mut rng);
    let sum = a.add(&b);
    
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("a + b = {:?}", sum);
}