/*
  Problem 6: Character Classifier

  Write a function that takes a char and returns a &'static str indicating whether it is
  "alphabetic", "numeric", "whitespace", or "other".

  Run the tests for this problem with:
    cargo test --test char_classifier_test
*/

pub fn classify_char(c: char) -> &'static str {
    // let ch = c as u8;
    // if (ch == 32) {
    //     return "whitespace";
    // }
    // if (ch == 10) {
    //     return "whitespace";
    // }
    // if (ch == 9) {
    //     return "whitespace";
    // }
    // for i in 65..91 {
    //     if (ch == i) {
    //         return "alphabetic";
    //     }
    // }
    // for i in 97..123 {
    //     if (ch == i) {
    //         return "alphabetic";
    //     }
    // }
    // for i in 48..58 {
    //     if (ch == i) {
    //         return "numeric";
    //     }
    // }
    // return "other";

    if c.is_alphabetic() {
      "alphabetic"
    }else if c.is_numeric() {
        "numeric"
    }else {
        "other"
    }
}
