


fn main() {
  println!( "{}", sum_multiples_3or5_less_than(10));
  println!( "{}", sum_multiples_3or5_less_than(1000));
}

fn sum_multiples_3or5_less_than(lim: i64) -> i64 {
  let mut sum:i64 = 0;
  for i in 0..lim {
    if i%3==0 || i%5==0 {
      sum += i;
    }
  }
  return sum;
}
