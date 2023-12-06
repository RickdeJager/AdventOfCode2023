use day06::{part1, part2, read_input};

fn main() {
    let input = read_input("input.txt"); // (3.5390 µs)
    let res = part1(&input);
    println!("Part 1: {}", res); //  (124.45 ns)

    let res = part2(&input);
    println!("Part 2: {}", res); //  (190.15 ns)
}
