# my_result_rs

A custom implementation of the `Result` type called `MyResult`, designed for learning and experimenting with error handling in Rust.

---

## Features

- Simple enum `MyResult<T, E>` with variants `Ok(T)` and `Err(E)`
- Commonly used methods like `map`, `and_then`, `unwrap`, `unwrap_or`
- Macros for ergonomic usage: `my_ok!`, `my_err!`, `my_try!`
- Implementations of `From` for easy conversion to/from `std::result::Result`
- Designed for educational purposes and to understand Rust’s error handling internals

---

## Example Usage

```rust
use my_result_rs::{my_ok, my_try};
use my_result_rs::my_result::MyResult;

fn add(x: i32, y: i32) -> MyResult<i32, &'static str> {
    my_ok!(x + y)
}

fn do_calc() -> MyResult<i32, &'static str> {
    let a = my_try!(add(2, 3));
    let b = my_try!(add(a, 4));
    my_ok!(b * 2)
}

fn main() {
    match do_calc() {
        MyResult::Ok(value) => println!("Calculation result: {}", value),
        MyResult::Err(err)   => println!("Calculation error: {}", err),
    }
}
````

---

## Conversion between `MyResult` and `std::result::Result`

You can easily convert between `MyResult<T, E>` and the standard `Result<T, E>` thanks to the implemented `From` traits:

```rust
let my_res: MyResult<i32, &str> = Ok(42).into(); // std Result -> MyResult
let std_res: Result<i32, &str> = my_res.into();  // MyResult -> std Result
```

This improves interoperability with Rust’s standard library and ecosystem.

---

## 🧠 Differences Compared to `std::result::Result`

| Feature                  | `std::result::Result`          | `MyResult`               |
| ------------------------ | ------------------------------ | ------------------------ |
| Ergonomics (traits)      | Full (e.g. `Iterator`, `From`) | None yet                 |
| Integration with `?`     | Yes (`Try` trait)              | No (unless implemented)  |
| For debugging / learning | Excellent                      | Intentionally simplified |

---

## Notes

* **Why no support for `?`?**
  `MyResult` does not implement the `Try` trait, so you cannot use the `?` operator yet. This is intentional to keep the implementation simple and easy to understand.

* **Is `MyResult` production-ready?**
  No. It is simplified for educational use. Use the standard `Result` type in production code.

* **Plans for future improvements?**
  Possibly! Adding more traits and better ergonomics is an option, but the current focus is on clarity.

* **Can I extend `MyResult`?**
  Absolutely! The code is open and designed for experimentation and learning.

---

## License

This project is licensed under the MIT License.

---

### Feel free to open issues or submit pull requests if you want to contribute or suggest improvements!
