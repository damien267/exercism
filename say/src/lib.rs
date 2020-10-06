pub fn encode(m: u64) {
  
//    let m: u64 = "14";
//    let n_twenty_three: &str = "23";
//    let mut return_string = String::new();
    let n: String = m.to_string();
    //I converted to string so that I could find the length immediately. 
    //Once the length is known, you can know the correct format. 
    if n.len() != 0 {

        if n.len() == 1 {find_ones(&n);}

        if n.len() == 2 {find_tens(&n);}

    }
}
        pub fn find_ones(n: &str) {
        
            match n {
                "1" => println!("one"),
                "2" => println!("two"),
                "3" => println!("three"),
                "4" => println!("four"),
                "5" => println!("five"),
                "6" => println!("six"),
                "7" => println!("seven"),
                "8" => println!("eight"),
                "9" => println!("nine"),
                 _  => println!("Input not read"),
                
            }
        }    
  

        pub fn find_tens(n: &str) {
                
            match n {
                "10" => println!("ten"),
                "11" => println!("eleven"),
                "12" => println!("twelve"),
                "13" => println!("thirteen"),
                "14" => println!("fourteen"),
                "15" => println!("fifteen"),
                "16" => println!("sixteen"),
                "17" => println!("seventeen"),
                "18" => println!("eighteen"),
                "19" => println!("nineteen"),
                "20" => println!("twenty"),
                "30" => println!("thirty"),
                "40" => println!("forty"),
                "50" => println!("fifty"),
                "60" => println!("sixty"),
                "70" => println!("seventy"),
                "80" => println!("eighty"),
                "90" => println!("ninety"),
                 _   => println!("Input not read"),
            }
            
        }


    //unimplemented!("Say {} in English.", n);

//I wanted to use enums, but couldnt figure out how to incorporate them.
            
/*             
const Zero_to_nineteen: [&str; 20] = [   
      "0" = "Zero",
      "1" = "One",
      "2" = "Two",
      "3" = "Three",
      "4" = "Four",
      "5" = "Five",
      "6" = "Six",
      "7" = "Seven",
      "8" = "Eight",
      "9" = "Nine",
      "10" = "Ten",
      "11" = "Eleven",
      "12" = "Twelve",
      "13" = "Thirteen",
      "14" = "Fourteen",
      "15" = "Fifteen",
      "16" = "Sixteen",
      "17" = "Seventeen",
      "18" = "Eighteen",
      "19" = "Nineteen",
];

const Twenty_to_ninety: [&str; 8] = [
      "20" = "Twenty",
      "30" = "Thirty",
      "40" = "Forty",
      "50" = "Fifty",
      "60" = "Sixty",
      "70" = "Seventy",
      "80" = "Eighty",
      "90" = "Ninety" 
];
      
     
*/      
    



//    return_string
//}    



