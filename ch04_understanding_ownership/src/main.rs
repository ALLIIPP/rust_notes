fn main() {
   /* let mut s = String::from("Hello");
    s.push_str(", World");

    println!("{}",s);*/

    let s1 = String::from("this is a sentence");
    let first = first_word(&s1);

    println!("the first word is: {}", first);
}


fn first_word(s:&str) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}
