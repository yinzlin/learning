fn main() {
    let mut s = String::from("hello");
println!("len: {}, capacity: {}", s.len(), s.capacity());  // len: 5, capacity: 5

s.push_str(" world!");
println!("len: {}, capacity: {}", s.len(), s.capacity());  // len: 12, capacity: 12 或更大
}