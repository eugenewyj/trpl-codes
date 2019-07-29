fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&mut s1);

    println!("The length of {} is {}.", s1, len);

    let mut s = String::from("hello world");

    let word = first_word(&s);

    println!("word is {}", word);

    let hello = first_word_str(&s);

    println!("hello is {}", hello);

    s.clear();
}

fn calculate_length(s: &mut String) -> usize {
    s.push_str("aa");
    return s.len();
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

fn first_word_str(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}