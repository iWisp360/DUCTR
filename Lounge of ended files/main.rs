use std::io;

fn main() {
    let mut s = String::new();
    println!("Tell your name with your last name: ");
    io::stdin().read_line(&mut s).expect("Hell Boy...");
    println!("Your first name is: {}", get_first_name(&s));
}

fn get_first_name(s: &String) -> String {
    let first_word_i = first_word(s);
    let first_word = &s[0..first_word_i];
    String::from(first_word)
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
