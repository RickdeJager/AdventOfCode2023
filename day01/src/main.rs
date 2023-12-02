use day01::{part1, part2, part2a, read_input};

fn main() {
    let input = read_input("input.txt");
    let res = part1(&input);
    println!("Part 1: {}", res); // 53651 (7.9387 µs)

    let res = part2(&input);
    println!("Part 2: {}", res); // 53894 (876.00 µs)

    let res: usize = part2a(&input);
    println!("Part 2a: {}", res); // 53894 (16.828 µs)
}
