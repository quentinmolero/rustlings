// move_semantics7.rs
// Make me compile and run without error !
// You can't declare new variable
// Use `rustlings hint move_semantics8` for hints

// I AM *NOT* DONE

fn main() {
    let mut v = Vec::new();
    add_one_value(/**/ &mut /**/ add_one_value(&mut v));
    assert_eq!(2, v.len()); // the size of the vector should be 2
}

fn add_one_value(v: &mut Vec<i32>) -> /**/ &mut /**/ Vec<i32> {
    // Do not change the body of this function
    v.push(1);
    v
}

