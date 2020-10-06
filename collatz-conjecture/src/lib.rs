pub fn collatz(n: u64) -> u64 {  //-> Option<u64>  I could not figure out how to return this. So I just return the integer. This way it builds.

  let mut num_in = n;
  let mut count = 0;

  match n {
      0 => panic!("Entered 0"),
      _ => {
             while num_in != 1 {
                 if num_in % 2 == 0 {
                     num_in = num_in / 2;
                 }
                 else {
                    num_in = num_in * 3 + 1;
                 }
                count = count + 1;
             }   
             count
           }
  }    





//    println!("return Some(x) where x is the number of steps required to reach 1 starting with {}", count )
}
