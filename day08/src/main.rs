use day08::{part1, part2, read_input};

fn main() {
    let input = read_input("input.txt"); // (46.478 µs)
    let res = part1(&input);
    println!("Part 1: {}", res); // 19951 (32.710 µs)

    let res = part2(&input);
    println!("Part 2: {}", res); // 16342438708751 (149.42 µs)
}
