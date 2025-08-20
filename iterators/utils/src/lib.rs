mod test;

pub mod dot_iter_types {
    pub fn iter_mut() {
        // Example of using iter_mut to modify a vector
        // This function will increment each element in the vector by 1
        // and print the modified vector.
        let mut variable = vec![1, 2, 3, 4, 5];
        let sum = variable.iter_mut();
        for value in sum {
            *value += 1; // Increment each value by 1
        }
        println!("Modified vector: {:?}", variable);
    }
    pub fn into_iter() {
        // This function is similar to iter_mut but uses into_iter
        // which consumes the vector and allows for modification.
        let variable = vec![1, 2, 3, 4, 5];
        let sum = variable.clone().into_iter();
        for value in sum {
            println!("{value}");
        }
        println!("{}", variable.len()); // This will not work as variable is consumed
    }
    pub fn iter() {
        // This function demonstrates using iter to iterate over a vector
        // without modifying it.
        // It will print each value in the vector.
        let variable = vec![1, 2, 3, 4, 5];
        let sum = variable.iter();
        for value in sum {
            println!("{value}"); // Increment each value by 1
        }
        println!("Modified vector: {:?}", variable);
    }
}

pub mod methods{
    pub fn method_consuming_iter(a:&Vec<i32>) -> i32 {
        println!("Sum of elements in the vector: {:?}", a);
        a.iter().sum()
    }
    pub fn method_producing_iters() -> Vec<i32> {
        let variable = vec![1, 2, 3, 4, 5];
        let new_array: Vec<_> = variable.iter().map(|x| x+2).collect();
        new_array
    }

}

pub mod just_tests {

    pub fn trial(){
        let v = vec![1, 2, 3, 4, 5];
        let sum = v.iter();
        println!("{}", sum.len());
        println!("iter: {:?}", sum);
        println!("Vector: {:?}", v);

        println!("iterating over the iter");
        for sum_value in sum.copied() {
            println!("Current sum: {}", sum_value);
        }
        println!("Iterating over the vector again:");
        for sum_value in v.iter().copied() {
            println!("Current sum: {}", sum_value);
        }

        let vec_str = ('a'..'z').map(String::from).collect::<Vec<String>>();

        for s in vec_str.iter() {
            let s = s as &str ;
            println!("s = {s}");
        }
    }

    pub fn looping_iter(v: &Vec<i32>) {
        v.iter().map(|x| x + 2).for_each(|x| println!("{}", x));
    }
}
