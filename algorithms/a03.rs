struct Friends{
    a:f32,
    b:f32,
    c:f32,
}


fn main() {
   
   let friend = Friends {a:10 as f32,b:15 as f32,c:9 as f32};
    splitthebill(friend);
}



fn splitthebill(friend:Friends){

    let sum:f32 = friend.a  +friend.b +friend.c ;
    let avg:f32 =sum/3  as f32;
    println!("avg price {}",avg);
    println!("a:{:.2},b:{:.2},c:{:.2}",friend.a- avg,friend.b-avg,friend.c-avg);
}