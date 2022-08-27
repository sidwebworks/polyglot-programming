pub fn get_input() -> &'static str {
    return "
		    forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
	";
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_line(line: &str) -> Point {
    let (direction, amount) = str::trim(line).split_once(" ").expect("Must contain a whitespace");

    if direction.is_empty() || amount.is_empty() {
        return Point { x: 0, y: 0 };
    }

    let amount = str::parse::<i32>(amount).expect("Second arg must be an integer");

    if direction == "forward" {
        return Point { x: amount, y: 0 };
    } else if direction == "up" {
        return Point { x: 0, y: -amount };
    }

    return Point { x: 0, y: amount };
}

pub fn run() {
    let result =
        str::trim(get_input())
            .lines()
            .map(parse_line)
            .fold(Point { x: 0, y: 0 }, |mut acc, point| {
                acc.x += point.x;
                acc.y += point.y;

                return acc;
            });

    println!("{:?} {1}", result, result.x * result.y)
}
