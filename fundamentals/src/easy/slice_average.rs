/*
  Problem 11: Slice Average

  Write a function that takes a slice of f64 values and returns the arithmetic mean.
  If the slice is empty, return None.

  Run the tests for this problem with:
    cargo test --test slice_average_test
*/

pub fn average(values: &[f64]) -> Option<f64> {
   let len = values.len();
   let mut sum:f64 = 0.0;
   if(len==0) {
     return None;
   }

   for i in 0..len{
    sum = sum + values[i];
   }

   let ans = sum/len as f64;
   return Some(ans) 
}
