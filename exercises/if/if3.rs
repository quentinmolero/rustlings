// if3.rs
// Step 1: Make me compile!
// Step 2: Get the tests passing!
// Execute `rustlings hint if3` for hints!

// I AM *NOT* DONE

fn compute_fees(price: i32) -> i32 {
    let fees: i32; // (mut not allowed)
    // * fees for price price lower than 10 is 1
    // * fees for price price higher than 10 is 10% rounded down (use / 10)
    // don't change before that line
    if price < 10 {
        fees = 1;
    } else {
        /**/        fees = price / 10
        /**/
    }
    // don't change after that line
    fees
}

// Don't modify this function!
#[test]
fn verify_fees() {
    assert_eq!(1, compute_fees(2));
    assert_eq!(1, compute_fees(10));
    assert_eq!(1, compute_fees(19));
    assert_eq!(4, compute_fees(42));
}



