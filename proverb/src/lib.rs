pub fn build_proverb(list: &[&str]) -> String {
/*
    let mylist = vec!["nail", "shoe", "horse", "rider", "message", "battle", "kingdom"];

    let mut answer = String::new();
	    
    let mut i = 0;
    let list_length = list.len();

    while i < list_length - 1 {
      // For want of a nail the shoe was lost.
      answer = println!("For want of a {} the {} was lost.", list.get(i).unwrap(), list.get(i+1).unwrap());
      i += 1;
      answer;
    }
    answer = println!("And all for the want of a {}.", list.get(i).unwrap());
    return answer;
}


fn main() {
*/
        let mylist = vec!["nail", "shoe", "horse", "rider", "message", "battle", "kingdom"];
    let mut answer_string = String::new();
    let mut i = 0;
    let list_length = list.len();

    while i < list_length {
      if i == list_length - 1 {
        // And all for the want of a nail.
        answer_string += &format!("And all for the want of a {}.", list.get(0).unwrap()); //+=&format! from https://github.com/apiraino/rust-exorcism/blob/master/proverb/src/lib.rs
        i += 1;
      }
      else {
        // For want of a nail the shoe was lost.
        answer_string += &format!("For want of a {} the {} was lost.\n", list.get(i).unwrap(), list.get(i+1).unwrap());
        i += 1;
      }
    }
    answer_string
}
