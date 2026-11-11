
fn main() {
    println!("Hello, world!");
    let reference: *const i32 = &10 as *const i32;
    unsafe {
        println!("{}", *reference);
    }

    let value_array: [i32; 3] = [1, 20, 3];

    let values_array_ptr = value_array.as_ptr() as *mut i32;
    let single_value = &value_array[0] as *const i32;
    unsafe {
        // println!("single value: {single_value:?} with value of {:?}", *single_value);
        // println!("value array: {values_array_ptr:?} with value of: {:?}", *values_array_ptr);

        let new_value = values_array_ptr.add(1);
        println!("{}", *new_value);
        *new_value += 3;
        println!("{}", *new_value);


        let new_single_value = single_value.add(1);
        println!("{}", *new_single_value);
    }

    println!("slice {:?}", value_array);
}
