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
