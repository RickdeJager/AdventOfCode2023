use day05::{part1, part2, read_input};

fn main() {
    let input = read_input("input.txt"); // (21.172 µs)
    let res = part1(&input);
    println!("Part 1: {}", res); // 486613012 (660.14 ns)

    let res = part2(&input);
    println!("Part 2: {}", res); // 56931769 (10.715 µs)
}
