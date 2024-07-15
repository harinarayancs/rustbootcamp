fn magnitude(v: &[f64;3]) -> f64 {
    let mut sum = 0.0;
    
    for &x in v {
        sum += x * x;
    }
    
    return sum.sqrt()
  }
  
  fn normalize(v: &mut [f64; 3]) {
    let mag = magnitude(v);
    
    for elem in v.iter_mut() {
        *elem /= mag;
    }
  }
  
  // Use the following `main` to test your work.
  
  fn main() {
      println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));
  
      let mut v = [1.0, 2.0, 9.0];
      println!("Magnitude of {v:?}: {}", magnitude(&v));
      normalize(&mut v);
      println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
  }