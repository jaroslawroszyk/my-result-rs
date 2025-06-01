use my_result_rs::{my_result::MyResult, my_try::MyTry};

#[test]
fn test_into_result_preserves_value_and_type() {
    let ok_val: MyResult<i32, &str> = MyResult::Ok(100);
    let err_val: MyResult<i32, &str> = MyResult::Err("failure");

    assert_eq!(ok_val.clone().into_result(), ok_val);
    assert_eq!(err_val.clone().into_result(), err_val);
}

#[test]
fn test_from_ok_creates_ok_variant() {
    let val = 123;
    let result: MyResult<i32, &str> = <MyResult<i32, &str> as MyTry>::from_ok(val);
    assert!(matches!(result, MyResult::Ok(v) if v == 123));

    let val = String::from("success");
    let result: MyResult<String, &str> = <MyResult<String, &str> as MyTry>::from_ok(val.clone());
    assert!(matches!(result, MyResult::Ok(ref v) if v == &val));
}

#[test]
fn test_from_err_creates_err_variant() {
    let err = "error";
    let result: MyResult<i32, &str> = <MyResult<i32, &str> as MyTry>::from_err(err);
    assert!(matches!(result, MyResult::Err(e) if e == "error"));

    let err = String::from("fail");
    let result: MyResult<i32, String> = <MyResult<i32, String> as MyTry>::from_err(err.clone());
    assert!(matches!(result, MyResult::Err(ref e) if e == &err));
}

#[test]
fn test_type_safety() {
    let ok: MyResult<u8, String> = <MyResult<u8, String> as MyTry>::from_ok(255);
    let err: MyResult<u8, String> =
        <MyResult<u8, String> as MyTry>::from_err(String::from("error"));

    match ok {
        MyResult::Ok(v) => assert_eq!(v, 255),
        _ => panic!("Should be Ok variant"),
    }

    match err {
        MyResult::Err(e) => assert_eq!(e, "error"),
        _ => panic!("Should be Err variant"),
    }
}

#[test]
fn test_edge_cases() {
    let opt: Option<i32> = None;
    let result: MyResult<Option<i32>, &str> = <MyResult<Option<i32>, &str> as MyTry>::from_ok(opt);
    assert!(matches!(result, MyResult::Ok(None)));

    let result: MyResult<i32, &str> = <MyResult<i32, &str> as MyTry>::from_err("");
    assert!(matches!(result, MyResult::Err("")));
}
