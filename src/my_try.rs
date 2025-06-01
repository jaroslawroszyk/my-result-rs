use crate::my_result::MyResult;

pub trait MyTry {
    type OkType;
    type ErrType;

    fn into_result(self) -> MyResult<Self::OkType, Self::ErrType>;
    fn from_ok(v: Self::OkType) -> Self;
    fn from_err(e: Self::ErrType) -> Self;
}

impl<T, E> MyTry for MyResult<T, E> {
    type OkType = T;
    type ErrType = E;

    fn into_result(self) -> MyResult<Self::OkType, Self::ErrType> {
        self
    }

    fn from_ok(v: Self::OkType) -> Self {
        MyResult::Ok(v)
    }

    fn from_err(e: Self::ErrType) -> Self {
        MyResult::Err(e)
    }
}

