pub fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .split("\n")
            .map(&first_last_number)
            .sum::<u32>()
    );
}

fn first_last_number(line: &str) -> u32 {
    let mut first = 10;
    let mut second = 10;
    for v in line.chars() {
        match v.to_digit(10) {
            Some(val) => {
                if first == 10 {
                    first = val;
                }
                second = val;
            }
            None => continue,
        }
    }

    if first == 10 {
        return 0;
    }

    first * 10 + second
}
