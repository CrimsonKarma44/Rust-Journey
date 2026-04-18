fn main() {
    let result = gcd_of_strings(
        "ABABABABAB".to_string(),
        "ABABABABAB".to_string(),
    );
    println!("{}", result);
}
fn gcd_of_strings(str1: String, str2: String) -> String {
    let mut repeated_val = String::from("");
    let mut index = 0;
    let mut match_index: Option<usize> = None;
    for i in str1.chars() {
        match match_index {
            Some(_) => {
                if index < str2.len() - 1 {
                    if i == *(&str2[index + 1..index + 2].chars().next().unwrap()) {
                        repeated_val.push_str(i.to_string().as_str());
                        index += 1;
                        continue;
                    } else {
                        break;
                    }
                }
            }
            None => {
                for (pos, v) in str2.chars().enumerate() {
                    if i == v {
                        match_index = Some(pos);
                        index = pos;
                        repeated_val.push_str(v.to_string().as_str());
                        break;
                    } else {
                        continue;
                    }
                }
            }
        }
    }
    let mut repeated_length = repeated_val.len();
    if match_index != None {
        loop {
            println!("{repeated_length}");
            if repeated_length < 1 {
                return String::from("");
            }
            if str1.len() % repeated_length == 0 {
                let mut condition = true;
                for i in (0..str1.len()).step_by(repeated_length) {
                    for v in 0..repeated_length {
                        if &repeated_val[v..v + 1] == &str1[v + i..v + i + 1] {
                            continue;
                        } else {
                            condition = false;
                            break;
                        }
                    }
                }
                if condition == true {
                    if str1 == (&repeated_val[0..repeated_length]).to_string() {
                        return bare(&repeated_val[0..repeated_length]);
                    } else {
                        return (&repeated_val[0..repeated_length]).to_string();
                    }
                } else {
                    repeated_length -= 1;
                }
            } else {
                repeated_length -= 1;
                continue;
            }
        }
    }
    return String::from("");
}

fn bare(value: &str) -> String {
    let first_index = &value[0..1];
    let mut result = String::from("");
    for i in 1..=value.len() {
        if result.len() > 1 {
            if first_index != &value[i - 1..i] {
                result.push_str(&value[i - 1..i]);
            } else {
                break;
            }
        } else {
            result.push_str(&value[i - 1..i]);
        }
    }
    result
}
