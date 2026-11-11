fn sort_words_by_embedded_number(input: &str) -> Vec<&str> {
    let mut words: Vec<&str> = input.split_whitespace().collect();

    words.sort_by_key(|word| {
        // Extract all digits from the word and parse as a number
        let num_str: String = word.chars().filter(|c| c.is_ascii_digit()).collect();
        num_str.parse::<u32>().unwrap_or(u32::MAX)
    });

    words
}

fn main() {
    let input = "worl1d peter4 hel2lo";
    let sorted = sort_words_by_embedded_number(input);
    println!("{sorted:?}");

    for word in sorted {
        println!("{}", word);
    }
    // Output:
    // worl1d
    // hel2lo
    // peter4

    let values = sortter(&mut vec![3, 10, 4, 2]);
    println!("{values:?}");
}

fn sortter(value: &mut Vec<i32>) -> Vec<i32> {
    let mut sorted = Vec::<i32>::new();
    let n = value.len() - 1;

    for _ in (0..=n).rev() {
        let mut max = 0;
        let mut pox = 0;

        for (y, x) in value.iter().enumerate() {
            if *x > max {
                max = *x;
                pox = y;
            }
        }

        value.remove(pox);

        println!("max: {max:?}");
        sorted.insert(0, max);
    }

    sorted
}
