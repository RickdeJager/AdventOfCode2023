use day11::{part1, part2, read_input};

fn main() {
    let input = read_input("input.txt"); // (41.080 µs)
    let res = part1(&input);
    println!("Part 1: {}", res); // 9609130 (2.5321 ms)

    let res = part2(&input);
    println!("Part 2: {}", res); // 702152204842 (2.6362 ms)
}
