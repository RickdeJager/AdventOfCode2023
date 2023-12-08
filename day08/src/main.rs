use day08::{part1, part2, read_input};

fn main() {
    let input = read_input("input.txt"); // ()
    let res = part1(&input);
    println!("Part 1: {}", res); // 19951 ()

    let res = part2(&input);
    println!("Part 2: {}", res); // 16342438708751 ()
}
