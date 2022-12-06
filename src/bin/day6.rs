
fn is_unique_chars(input: &[char]) -> bool {
    println!("{:?}", input);
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i != j && input[i] == input[j] {
                return false;
            }
        }
    }
    true
}

fn main() {
    // let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars().collect::<Vec<_>>();
    let input = include_str!("day6.txt").chars().collect::<Vec<_>>();

    // Part 1
    for i in 4..input.len() {
        if is_unique_chars(&input[i - 4..i]) {
            println!("{}", i);
            break;
        }
    }

    // Part 2 - change 4 to 14, and it works...
    for i in 14..input.len() {
        if is_unique_chars(&input[i - 14..i]) {
            println!("{}", i);
            break;
        }
    }
}