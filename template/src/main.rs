use dayXX::{part1, part2, read_input};

fn main() {
    let input = read_input("input.txt");
    let res = part1(&input);
    println!("Part 1: {}", res);

    let res = part2(&input);
    println!("Part 2: {}", res);
}
