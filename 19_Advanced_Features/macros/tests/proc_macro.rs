use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[test]
fn test_hello_macro() {
    #[derive(HelloMacro)]
    struct MyStruct;

    MyStruct::hello_macro();
    assert!(true);
}

