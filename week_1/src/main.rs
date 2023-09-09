pub fn concatenate_strings(s1: &str, s2: &str) -> String {
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    result
}

fn main() {
    let string1 = "Hello";
    let string2 = "World";

    let concatenated_strings = concatenate_strings(string1, string2);

    println!("concatenated_strings: {}", concatenated_strings);
}

#[test]
fn should_concatenate_two_strings() {
    let s1 = "Rise";
    let s2 = "In";
    let result = concatenate_strings(s1, s2);
    assert_eq!(result, String::from("RiseIn"));
}
