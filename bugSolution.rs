fn main() {
    let mut v = vec![1, 2, 3];
    // Safe alternative: Use indexing
    v[0] = 10;
    println!("v: {:?}", v); 
} 