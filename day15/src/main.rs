use day15::{part1, part2, read_input};

fn main() {
    let input = read_input("input.txt"); // (5.3503 µs)
    let res = part1(&input);
    println!("Part 1: {}", res); // 516804 (26.766 µs)

    let res = part2(&input);
    println!("Part 2: {}", res); // 231844 (84.682 µs)
}
