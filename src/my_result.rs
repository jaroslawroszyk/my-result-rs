use std::fmt;

#[derive(PartialEq, Eq, Clone)]
pub enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> MyResult<T, E> {
    pub fn is_ok(&self) -> bool {
        matches!(self, MyResult::Ok(_))
    }

    pub fn is_err(&self) -> bool {
        matches!(self, MyResult::Err(_))
    }

    pub fn ok(self) -> Option<T> {
        match self {
            MyResult::Ok(v) => Some(v),
            MyResult::Err(_) => None,
        }
    }

    pub fn err(self) -> Option<E> {
        match self {
            MyResult::Ok(_) => None,
            MyResult::Err(e) => Some(e),
        }
    }

    pub fn unwrap(self) -> T {
        match self {
            MyResult::Ok(v) => v,
            MyResult::Err(_) => panic!("called `MyResult::unwrap()` on an `Err` value"),
        }
    }

    pub fn unwrap_or(self, default: T) -> T {
        match self {
            MyResult::Ok(v) => v,
            MyResult::Err(_) => default,
        }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> MyResult<U, E> {
        match self {
            MyResult::Ok(v) => MyResult::Ok(f(v)),
            MyResult::Err(e) => MyResult::Err(e),
        }
    }

    pub fn map_err<F2, F: FnOnce(E) -> F2>(self, f: F) -> MyResult<T, F2> {
        match self {
            MyResult::Ok(v) => MyResult::Ok(v),
            MyResult::Err(e) => MyResult::Err(f(e)),
        }
    }

    pub fn and_then<U, F: FnOnce(T) -> MyResult<U, E>>(self, f: F) -> MyResult<U, E> {
        match self {
            MyResult::Ok(v) => f(v),
            MyResult::Err(e) => MyResult::Err(e),
        }
    }
}

impl<T, E> From<MyResult<T, E>> for Result<T, E> {
    fn from(mr: MyResult<T, E>) -> Self {
        match mr {
            MyResult::Ok(v) => Ok(v),
            MyResult::Err(e) => Err(e),
        }
    }
}

impl<T, E> From<Result<T, E>> for MyResult<T, E> {
    fn from(res: Result<T, E>) -> Self {
        match res {
            Ok(v) => MyResult::Ok(v),
            Err(e) => MyResult::Err(e),
        }
    }
}

impl<T: fmt::Debug, E: fmt::Debug> fmt::Debug for MyResult<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyResult::Ok(val) => f.debug_tuple("MyResult::Ok").field(val).finish(),
            MyResult::Err(err) => f.debug_tuple("MyResult::Err").field(err).finish(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ok_and_is_err() {
        let ok: MyResult<i32, &str> = MyResult::Ok(42);
        let err: MyResult<i32, &str> = MyResult::Err("fail");

        assert!(ok.is_ok());
        assert!(!ok.is_err());

        assert!(err.is_err());
        assert!(!err.is_ok());
    }

    #[test]
    fn test_ok_and_err_methods() {
        let ok: MyResult<&str, &str> = MyResult::Ok("value");
        let err: MyResult<&str, &str> = MyResult::Err("error");

        assert_eq!(ok.clone().ok(), Some("value"));
        assert_eq!(ok.err(), None);

        assert_eq!(err.clone().ok(), None);
        assert_eq!(err.err(), Some("error"));
    }

    #[test]
    fn test_unwrap() {
        let ok: MyResult<i32, &str> = MyResult::Ok(123);
        assert_eq!(ok.unwrap(), 123);
    }

    #[test]
    #[should_panic(expected = "called `MyResult::unwrap()` on an `Err` value")]
    fn test_unwrap_panic() {
        let err: MyResult<i32, &str> = MyResult::Err("nope");
        let _ = err.unwrap();
    }

    #[test]
    fn test_unwrap_or() {
        let ok: MyResult<i32, &str> = MyResult::Ok(1);
        let err: MyResult<i32, &str> = MyResult::Err("fail");

        assert_eq!(ok.unwrap_or(999), 1);
        assert_eq!(err.unwrap_or(999), 999);
    }

    #[test]
    fn test_map() {
        let ok: MyResult<i32, &str> = MyResult::Ok(5);
        let err: MyResult<i32, &str> = MyResult::Err("fail");

        let mapped_ok = ok.map(|v| v * 2);
        let mapped_err = err.map(|v| v * 2);

        assert_eq!(mapped_ok, MyResult::Ok(10));
        assert_eq!(mapped_err, MyResult::Err("fail"));
    }

    #[test]
    fn test_map_err() {
        let ok: MyResult<i32, &str> = MyResult::Ok(7);
        let err: MyResult<i32, &str> = MyResult::Err("fail");

        let mapped_ok = ok.map_err(|e| format!("e: {}", e));
        let mapped_err = err.map_err(|e| format!("e: {}", e));

        assert_eq!(mapped_ok, MyResult::Ok(7));
        assert_eq!(mapped_err, MyResult::Err("e: fail".to_string()));
    }

    #[test]
    fn test_and_then() {
        fn square_if_positive(x: i32) -> MyResult<i32, &'static str> {
            if x >= 0 {
                MyResult::Ok(x * x)
            } else {
                MyResult::Err("negative number")
            }
        }

        let ok = MyResult::Ok(4);
        let err = MyResult::Err("initial fail");

        assert_eq!(ok.and_then(square_if_positive), MyResult::Ok(16));
        assert_eq!(
            err.and_then(square_if_positive),
            MyResult::Err("initial fail")
        );
    }

    #[test]
    fn test_debug_impl_for_ok() {
        let val: MyResult<i32, &str> = MyResult::Ok(42);
        let debug_str = format!("{:?}", val);
        assert_eq!(debug_str, "MyResult::Ok(42)");
    }

    #[test]
    fn test_debug_impl_for_err() {
        let val: MyResult<i32, &str> = MyResult::Err("error");
        let debug_str = format!("{:?}", val);
        assert_eq!(debug_str, "MyResult::Err(\"error\")");
    }

    #[test]
    fn test_myresult_into_result_ok() {
        let my_res: MyResult<i32, &str> = MyResult::Ok(10);
        let std_res: Result<i32, &str> = my_res.into();
        assert_eq!(std_res, Ok(10));
    }

    #[test]
    fn test_myresult_into_result_err() {
        let my_res: MyResult<i32, &str> = MyResult::Err("error");
        let std_res: Result<i32, &str> = my_res.into();
        assert_eq!(std_res, Err("error"));
    }

    #[test]
    fn test_result_into_myresult_ok() {
        let std_res: Result<i32, &str> = Ok(20);
        let my_res: MyResult<i32, &str> = std_res.into();
        assert_eq!(my_res, MyResult::Ok(20));
    }

    #[test]
    fn test_result_into_myresult_err() {
        let std_res: Result<i32, &str> = Err("fail");
        let my_res: MyResult<i32, &str> = std_res.into();
        assert_eq!(my_res, MyResult::Err("fail"));
    }
}
