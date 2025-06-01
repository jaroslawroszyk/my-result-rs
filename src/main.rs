use my_result_rs::my_result::MyResult;
use my_result_rs::{my_ok, my_try};

fn divide(a: f64, b: f64) -> MyResult<f64, String> {
    if b == 0.0 {
        MyResult::Err("Cannot divide by zero".to_string())
    } else {
        MyResult::Ok(a / b)
    }
}

fn add(x: i32, y: i32) -> MyResult<i32, &'static str> {
    my_ok!(x + y)
}

fn do_calc() -> MyResult<i32, &'static str> {
    add(2, 3)
        .and_then(|a| add(a, 4))
        .and_then(|b| my_ok!(b * 2))
}

fn do_calc2() -> MyResult<i32, &'static str> {
    let a = my_try!(add(2, 3));
    let b = my_try!(add(a, 4));
    my_ok!(b * 2)
}

fn main() {
    let result = divide(10.0, 2.0)
        .map(|r| r * 100.0)
        .and_then(|r| divide(r, 2.0));

    match result {
        MyResult::Ok(v) => println!("Final result: {}", v),
        MyResult::Err(e) => println!("Failure: {}", e),
    }

    match do_calc() {
        MyResult::Ok(value) => println!("Calculation result: {}", value),
        MyResult::Err(err) => println!("Calculation error: {}", err),
    }

    match do_calc2() {
        MyResult::Ok(value) => println!("Calculation result: {}", value),
        MyResult::Err(err) => println!("Calculation error: {}", err),
    }
}
