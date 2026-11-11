pub fn find_largest_in_slice(slice: &[i32]) -> Option<i32> {
    // Your code here...
    if slice.len() == 0 {
        None
    } else {
        let mut largest:i32 = slice[0];
        for i in slice.iter(){
            if *i > largest{
                largest = *i;
            } else if largest >= *i {
                continue;
            }
        }
        Some(largest)
    }

}

// Example Usage
pub fn main() {
    let numbers = [1, 3, 7, 2, 5];
    assert_eq!(find_largest_in_slice(&numbers), Some(7));

    let empty: [i32; 0] = [];
    assert_eq!(find_largest_in_slice(&empty), None);

    let single_element = [42];
    assert_eq!(find_largest_in_slice(&single_element), Some(42));
}
