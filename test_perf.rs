fn main() {
    let mut s1 = String::with_capacity(100);
    s1.push_str("hello\n\nworld");
    let pos = s1.find("\n\n").unwrap();
    let block = s1[..pos].to_string();
    s1 = s1[pos + 2..].to_string();
    println!("s1 cap: {}", s1.capacity());

    let mut s2 = String::with_capacity(100);
    s2.push_str("hello\n\nworld");
    let pos = s2.find("\n\n").unwrap();
    let block = s2[..pos].to_string();
    s2.drain(..pos + 2);
    println!("s2 cap: {}", s2.capacity());
}
