pub mod my_result;
pub mod my_try;

#[macro_export]
macro_rules! my_ok {
    ($val:expr) => {
        $crate::my_result::MyResult::Ok($val)
    };
}

#[macro_export]
macro_rules! my_err {
    ($err:expr) => {
        $crate::my_result::MyResult::Err($err)
    };
}

#[macro_export]
macro_rules! my_try {
    ($expr:expr) => {
        match $expr {
            $crate::my_result::MyResult::Ok(val) => val,
            $crate::my_result::MyResult::Err(err) => return $crate::my_result::MyResult::Err(err),
        }
    };
}
