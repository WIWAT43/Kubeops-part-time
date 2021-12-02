
fn main() {
    distribute(3, 13);
    distribute(4, 1);
    distribute(2, 4);
    distribute(3, 3);
    distribute(4,10);
    }
    
    fn distribute(server:i32,jobs:i32){
    
        let _server_load =jobs/server;
        let mut _extra_job = jobs%server;
        let mut count =0;
        let mut ext;
       
     print!("[");
        for i in 0..server {
            print!("[");
            
            if _extra_job>0 {
               ext=1; 
               _extra_job =_extra_job-1;
            }else {
                ext=0;  
            }
         
           for j in 0.._server_load+ext{
            match j {0=>print!("{}",count), _=>print!(",{}",count)};
            count = count+1;
           }
         if i+1 == server{
             print!("]");  
          }
          else {
           print!("],");  
         }
           
       
        }
         println!("]");
    }