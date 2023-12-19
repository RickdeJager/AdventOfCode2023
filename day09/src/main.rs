use day09::{part1, part2, read_input};

fn main() {
    let input = read_input("input.txt"); // (60.938 µs)
    let res = part1(&input);
    println!("Part 1: {}", res); // 1955513104 (27.388 µs)

    let res = part2(&input);
    println!("Part 2: {}", res); // 1131 (28.197 µs)
}
