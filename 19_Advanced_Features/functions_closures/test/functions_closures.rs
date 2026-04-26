use functions_closures::{
    advanced_functions_closures::{
        fn_function_pointers::{Accept, Status, add_one, do_twice},
        returing_closures::{
            returning_closure_from_fn, returning_closure_from_fn_mut, returning_closure_impl,
        },
    },
    experiment::closure_test,
};

#[test]
fn experiment_test() {
    let mut word1 = String::from("abc");
    assert_eq!(
        closure_test(|x| {
            word1.push_str(x);
            x.push_str("defg");
            x.len()
        }),
        7
    );
}

#[test]
fn test_fn_function_pointer() {
    assert_eq!(do_twice(5, add_one), 12);
}

#[test]
fn test_accept_fn_closure() {
    let accept = Accept::new(5);
    assert!(accept.conv_status(|x| Status::Value(x)) == Status::Value(5));
    // also works: functions
    assert!(accept.conv_status(status_from_u32) == Status::Value(5));
    // also works: enums
    assert!(accept.conv_status(Status::Value) == Status::Value(5));
}

fn status_from_u32(x: u32) -> Status {
    Status::Value(x)
}

#[test]
fn closure_from() {
    assert_eq!(returning_closure_from_fn()(), 42);
    assert_eq!(returning_closure_from_fn_mut()(), 42);
    assert_eq!(returning_closure_impl()(), 42);
}
