use day12::{part1, part2, read_input};

fn main() {
    let input = read_input("input.txt");
    let res = part1(&input);
    println!("Part 1: {}", res); // 7163 (86.203 µs)

    let res = part2(&input);
    println!("Part 2: {}", res); // 17788038834112 (371.34 µs)
}
