pub fn dec1(input: String) -> String {
    let mut output = 50;
    let instructions: Vec<_> = input.lines().collect();
    let mut counter = 0;
    for i in instructions {
        let without_first = &i[1..];
        let previous = output;
        if i.starts_with("L") {
            output -= without_first.parse::<i32>().unwrap();
        } else if i.starts_with("R") {
            output += without_first.parse::<i32>().unwrap();
        }

        counter += (output / 100).abs();

        if output == 0 {
            counter += 1;
        } else if output < 0 && previous != 0 {
            counter += 1;
        }

        output = output.rem_euclid(100);
        println!("{}", output);
    }
    return counter.to_string();
}
