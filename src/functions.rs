pub fn find_pattern(target:String,subject:String) -> bool {
    let target = string_to_vec(target);
    let subject = string_to_vec(subject);

    println!("{:?}",target);
    println!("{:?}",subject);

    for idx in 0..(subject.len()-target.len()) {
        if target == subject[idx..idx+target.len()] {
            return true
        }
    };
    
    return false
}

fn string_to_vec(string:String) -> Vec<String> {
    let mut string : Vec<&str> = string.split("").collect();
    string.remove(0);
    string.pop();
    return string.into_iter().map(|x| x.to_owned()).collect::<Vec<String>>();
}

//fn find_dynamic_pattern() 
#[derive(Debug)]
pub enum char_types {
    upper,
    lower,
    num,
    symbol
}

pub fn return_char_type(char:String) -> char_types {
    let lower = string_to_vec("abcdefghijklmnopqrstuvwxyz".to_owned());
    let upper = string_to_vec("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned());
    let numbers = string_to_vec("1234567890".to_owned());
    if lower.contains(&char) {
        return char_types::lower
    } else if upper.contains(&char) {
        return char_types::upper
    } else if numbers.contains(&char) {
        return char_types::num
    } else {
        return char_types::symbol
    }
}