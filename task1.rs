fn collatz_length(mut n: i32) -> u32 {
    let mut length : u32 = 1;
    while n != 1 {
    if n%2==0{
        n = n/2;
    }
    else {
        n = 3*n+1;
    }
    length += 1;
    }
    return length;
  }
  
  fn main() {
    let n: i32 = 3;
    println!("Length of collatz is {}",collatz_length(n));
  }