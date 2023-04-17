// if1.rs
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.


use std::cmp::Ordering;

pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables

    // match a.cmp(&b) {
    //     Ordering::Greater => a,
    //     Ordering::Less => b,
    //     Ordering::Equal => a,
    // }

    if a > b {
        a
    } else {
        b
    }
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
