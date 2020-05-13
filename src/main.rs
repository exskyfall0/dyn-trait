fn main() {}

trait MyErr {
  fn shutdown(&self) {
    panic!("shutdown");
  }
}

impl<T> From<T> for Box<dyn MyErr>
where
  T: MyErr + 'static,
{
  fn from(e: T) -> Self {
    Box::new(e)
  }
}

struct Err1 {}
impl MyErr for Err1 {}

struct Err2 {}
impl MyErr for Err2 {}

// fn try_return_err(b: bool) -> Box<dyn MyErr> {
//     if b {
//         let e = Err1 {};
//     } else {
//         Err2 {}
//     }
// }
