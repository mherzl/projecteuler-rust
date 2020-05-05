


fn main() {
  let a1 = sum_evens_less_than(9);
  println!("{}", a1);
  let a2 = sum_evens_less_than(4000000);
  println!("{}", a2);
}

fn sum_evens_less_than(lim: u64) -> u64 {
  let mut ans = 0;
  let mut iter = fibonacci();
  let mut x = iter.next();
  while x < Some(lim) {
    match x {
      Some(xx) => ans += xx,
      None => return ans,
    }
    iter.next();
    iter.next();
    x = iter.next();
  }
  ans
}


struct Fibonacci {
  curr: u64,
  next: u64,
}

impl Iterator for Fibonacci {
  type Item = u64;
  fn next(&mut self) -> Option<u64> {
    let ans: u64 = self.curr;
    let new_next = self.curr + self.next;
    self.curr = self.next;
    self.next = new_next;
    Some(ans)
  }
}

fn fibonacci() -> Fibonacci {
  Fibonacci { curr: 0, next: 1 }
}




