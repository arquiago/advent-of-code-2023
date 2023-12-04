use aho_corasick::AhoCorasick;

fn main() {
    let input = std::fs::read_to_string("/home/archago/aoc/aoc1/aocinp.txt").unwrap();

    let patterns = &[
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    let ac = AhoCorasick::new(patterns).unwrap();

    let result = input
        .lines()
        .map(|line| {
            let mut matches = ac
                .find_overlapping_iter(line)
                .map(|m| m.pattern().as_usize());

            let tens = matches.next().unwrap() % 9;
            let units = matches.last().unwrap_or(tens) % 9;

            ((tens + 1) * 10 + units + 1) as u32
        })
        .sum::<u32>();

    println!("{result}");
}
