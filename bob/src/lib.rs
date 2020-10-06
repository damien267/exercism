pub fn reply(message: &str) -> &str {
  
  let answer: &str = message.trim();
  let mut reply: &str = &String::new();


    if answer.to_uppercase() == answer {reply = "Whoa, chill out!"}
    else if answer.chars().rev().take(1) == "?" && answer.to_uppercase() == answer {reply = "Calm down, I know what I'm doing!"} 
    else if answer == "" {reply = "Fine. Be that way!"}
    else if answer.chars().rev().take(1) == "?" {reply = "Sure."}
    else {reply = "Whatever."}

  
  reply

//  unimplemented!("have Bob reply to the incoming message: {}", message)
}
