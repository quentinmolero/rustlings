// move_semantics9.rs
// Make me compile only by reordering the existing lines in `main()`,
// but without adding, changing or removing anything else.
// `rustlings hint move_semantics9` for hints

fn main() {
    let mut x = "Hello".to_string();
    // handle* variables simulate handles of a detached
    // execution like in threads
    let handle1 = process_and_readonly_data(&x);
    println!("{}", handle1);
    let handle2 = process_and_readonly_data(&x);
    println!("{}", handle2);
    let handle3 = process_and_write_data(&mut x);
    println!("{}", handle3);
    let handle4 = process_and_write_data(&mut x);
    println!("{}", handle4);
}

fn process_and_write_data(x: &mut String) -> &mut String {
    /* can change data */
    x.push_str("!");
    x
}

fn process_and_readonly_data(x: &String) -> &String {
    /* anything that doesn't change data */
    x
}
