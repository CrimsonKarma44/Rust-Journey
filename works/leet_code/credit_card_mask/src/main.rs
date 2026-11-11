fn main() {
    println!("Hello, world!");
    let cc = "1234567890";
    println!("{}", maskify(cc));
}

/// Return a String with all characters masked as '#' except the last 4.
fn maskify(cc: &str) -> String {
    // let first = &cc[..cc.len().saturating_sub(4)];
    let first = &cc[..cc.len().saturating_sub(4)];
    let last = &cc[cc.len().saturating_sub(4)..];
    let masked = first.chars().map(|_| '#').collect::<String>();
    masked + last
}

// using repeat
fn maskify_2(cc: &str) -> String {
    if cc.len() > 4{
        "#".repeat(cc.len()-4) + &cc[cc.len() - 4..]
    }else {
        cc.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::maskify;

    #[test]
    fn it_masks_example_strings() {
        assert_eq!(maskify("4556364607935616"), "############5616");
        assert_eq!(maskify("1"), "1");
        assert_eq!(maskify("11111"), "#1111");
    }
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;

    #[test]
    fn it_handles_example_long_string() {
        assert_eq!(maskify("4556364607935616"), "############5616");
    }

    #[test]
    fn it_handles_example_short_string() {
        assert_eq!(maskify("1"), "1");
    }

    #[test]
    fn it_handles_example_medium_string() {
        assert_eq!(maskify("11111"), "#1111");
    }

    #[test]
    fn it_handles_empty_string() {
        assert_eq!(maskify(""), "");
    }

    #[test]
    fn it_handles_short_strings() {
        let num_strings = 100;
        let min_len = 1;
        let max_len = 4;
        
        for string in gen_random_strings(num_strings, min_len, max_len) {
            assert_eq!(maskify(&string), string);
        }
    }
    
    #[test]
    fn it_handles_random_strings() {
        let num_strings = 100;
        let min_len = 0;
        let max_len = 16;
        
        for string in gen_random_strings(num_strings, min_len, max_len) {
            match string.len() {
                0..=4 => assert_eq!(maskify(&string), string),
                _ => {
                    let expected = __maskify(&string);
                    assert_eq!(maskify(&string), expected);
                }
            }
        }
    }
    
    #[test]
    fn it_handles_100k_random_strings() {
        let num_strings = 100_000;
        let min_len = 0;
        let max_len = 16;
        
        for string in gen_random_strings(num_strings, min_len, max_len) {
            match string.len() {
                0..=4 => assert_eq!(maskify(&string), string),
                _ => {
                    let expected = __maskify(&string);
                    assert_eq!(maskify(&string), expected);
                }
            }
        }
    }
    
    fn random_string(min_len: usize, max_len: usize) -> String {
        let mut rng = thread_rng();
        let len = rng.gen_range(min_len..max_len);
        rng.sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }
    
    fn gen_random_strings(num_strings: usize, min_len: usize, max_len: usize) -> Vec<String> {
        let mut strings: Vec<String> = vec![];
        for _ in 0..num_strings {
            let string = random_string(min_len, max_len);
            strings.push(string);
        }
        strings
    }
    
    fn __maskify(cc: &str) -> String {
        let cc_len = cc.len();
        let mask = 4;
    
        if cc_len <= mask {
            return String::from(cc);
        }
        
        let mask_until = cc_len - mask;

        cc.chars().enumerate()
            .map(|(i, c)|
                if i < mask_until { '#' } else { c }
            )
            .collect()
    }
}