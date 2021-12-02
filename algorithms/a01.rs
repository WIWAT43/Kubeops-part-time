fn main() {
    fizzbuzz(21);
    fizzbuzz(25);
    fizzbuzz(45);
    
    }
    
fn fizzbuzz(input_no:i32){
  
println!("{}{}", match input_no%3{0=>"fizz", _=>""},match input_no%5{0=>"buzz", _=>""});
}