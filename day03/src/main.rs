use day03::{part1, part2, read_input};

fn main() {
    let input = read_input("input.txt");
    let res = part1(&input);
    println!("Part 1: {}", res); // 539713 (79.249 µs)

    let res = part2(&input);
    println!("Part 2: {}", res); // 84159075 (23.053 µs)
}
