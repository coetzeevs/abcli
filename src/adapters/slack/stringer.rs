pub fn reverse(input: &String) -> String {
    println!("{:?}", *input);
    input.chars().rev().collect()
}

pub fn inspect(input: &String, digits: bool) -> (i32, String) {
    if !digits {
        return (input.len() as i32, String::from("char"));
    }
    (inspect_numbers(input), String::from("digit"))
}

fn inspect_numbers(input: &str) -> i32 {
    let mut count = 0;
    for c in input.chars() {
        if c.is_ascii_digit() {
            count += 1;
        }
    }
    count
}
