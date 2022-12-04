// Solution for https://adventofcode.com/2022/day/1.
fn main() {
    let input = include_str!("day1.txt");

    let mut calory_counts: Vec<u32> = vec![];
    let mut current_total_calories = 0;

    for calories in input.lines() {
        match calories.parse::<u32>() {
            Ok(calories) => {
                current_total_calories += calories;
            }
            Err(_) => {
                calory_counts.push(current_total_calories);
                current_total_calories = 0;
            }
        }
    }

    calory_counts.sort();

    println!("Top Elf Calories: {}", calory_counts.last().unwrap());
    println!(
        "Top 3 Elves Calories: {}",
        calory_counts.iter().rev().take(3).sum::<u32>()
    );
}
