/*
  Problem 18: String Reversal

  Write a function that takes a &str and returns a new String with the characters reversed.
  Be careful with Unicode — use .chars().

  Run the tests for this problem with:
    cargo test --test reverse_string_test
*/

pub fn reverse_string(s: &str) -> String {
    // let mut vec = Vec::new();
    let mut rev = String::from("");
    // for i in s.chars() {
    //     vec.push(i);
    // } 
    // let len = vec.len();
    // for i in 0..len{
    //     rev.push(vec[len - 1 - i]);
    // }
    // return rev;

    for i in 0..s.chars().count() {
        rev.push(s.chars().nth(s.chars().count() - 1 - i).unwrap());
    }
    return rev;
}
