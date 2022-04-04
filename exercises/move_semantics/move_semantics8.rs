// move_semantics8.rs
// Make me compile and run without error !
// `rustlings hint move_semantics8` for hints

// I AM *NOT* DONE

fn non_null_ref(/**/ mut /**/ v: &i32) -> &i32 {
    if *v == 0 {
        // let b = 1;
        // v = &b;
        v = &1;
    }
    v // don't change this line
}

// Don't modify this function!
#[test]
fn non_null_ref0_is_ref1() {
    let a = non_null_ref(&0);
    assert_eq!(*a, 1); // these two assertions
    assert_eq!(a, &1); //   are equivalent
}

// Don't modify this function!
#[test]
fn non_null_ref0_is_same() {
    let b = non_null_ref(&1);
    assert_eq!(*b, 1);
    assert_eq!(b, &1);
}



