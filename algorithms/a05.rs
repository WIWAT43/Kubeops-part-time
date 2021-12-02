fn main() {
    palindrome("Ada");
    palindrome("ciic");
    palindrome("15651");
    palindrome("1561");
}

fn palindrome(text:&str){
    let text_vec: Vec<char> = text.to_uppercase().chars().collect();
   
    let mut b = true;

    for i in 0..text.len(){
       if text_vec[i] !=  text_vec[text.len()-i-1]{
            b=false;
       }
       
    }
    if b {
        println!("{}, is Palindrome.",text);
    }else 
    {
        println!("{}, is NOT Palindrome.",text);
    }
  

}