use day02::{part1, part2, read_input};

fn main() {
    let input = read_input("input.txt");
    let res = part1(&input);
    println!("Part 1: {}", res); // 1867 (180.02 ns)

    let res = part2(&input);
    println!("Part 2: {}", res); // 84538 (265.32 ns)
}
