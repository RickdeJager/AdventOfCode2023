use day16::{part1, part2, read_input};

fn main() {
    let input = read_input("input.txt"); // (20.778 µs)
    let res = part1(&input); // 7860 (58.225 µs)
    println!("Part 1: {}", res);

    let res = part2(&input); // 8331 (43.718 ms)
    println!("Part 2: {}", res);
}
