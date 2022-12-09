use crate::reader::read_lines;
use colored::Colorize;

pub fn day9() {
    let day = 9;
    println!(
        "{}{}{} ",
        "\nDay ".bold().bright_green(),
        day.to_string().bold().bright_red(),
        ":".bold().bright_green()
    );
    if let Ok(lines) = read_lines(format!("./data/day{}.txt", day)) {
        let mut head = (0, 0);
        let mut tail = (0, 0);
        let mut headlong = (0, 0);
        let mut between0 = (0, 0);
        let mut between1 = (0, 0);
        let mut between2 = (0, 0);
        let mut between3 = (0, 0);
        let mut between4 = (0, 0);
        let mut between5 = (0, 0);
        let mut between6 = (0, 0);
        let mut between7 = (0, 0);
        let mut taillong = (0, 0);
        let mut visited: Vec<(i32, i32)> = Vec::new();
        let mut visited2: Vec<(i32, i32)> = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                let split: Vec<&str> = ip.split(' ').collect();
                let dir = match split[0] {
                    "U" => (0, 1),
                    "R" => (1, 0),
                    "D" => (0, -1),
                    "L" => (-1, 0),
                    _ => panic!("ERROR"),
                };
                for _ in 0..split[1].parse::<u32>().unwrap() {
                    head.0 = head.0 + dir.0;
                    head.1 = head.1 + dir.1;

                    headlong.0 = headlong.0 + dir.0;
                    headlong.1 = headlong.1 + dir.1;

                    follow(&mut tail, head);

                    follow(&mut between0, headlong);
                    follow(&mut between1, between0);
                    follow(&mut between2, between1);
                    follow(&mut between3, between2);
                    follow(&mut between4, between3);
                    follow(&mut between5, between4);
                    follow(&mut between6, between5);
                    follow(&mut between7, between6);
                    follow(&mut taillong, between7);

                    let between = distance_between(head, tail);
                    //println!("HEAD: {}, {} | TAIL: {}, {} ==> {}, {}", head.0, head.1, tail.0, tail.1, between.0, between.1);
                    if !visited.contains(&tail) {
                        visited.push(tail);
                    }
                    if !visited2.contains(&taillong) {
                        visited2.push(taillong);
                    }
                }
            }
        }
        println!("{}", visited.iter().count().to_string().bright_white());
        println!("{}", visited2.iter().count().to_string().bright_yellow());
        /*let minX = visited.iter().map(|f| f.0).min().unwrap();
        let maxX = visited.iter().map(|f| f.0).max().unwrap();
        let minY = visited.iter().map(|f| f.1).min().unwrap();
        let maxY = visited.iter().map(|f| f.1).max().unwrap();
        println!("{}, {}, {}, {}", minX, maxX, minY, maxY);*/
        const minX: i32 = -126;
        const maxX: i32 = 70;
        const minY: i32 = -290;
        const maxY: i32 = 23;

        let mut map = [['.'; (maxX + minX.abs()) as usize]; (maxY + minY.abs()) as usize];
        for pos in visited2 {
            if (pos.0 == 0 && pos.1 == 0) {
                map[(pos.1 + minY.abs()) as usize][(pos.0 + minX.abs()) as usize] = 'S';
            } else {
                map[(pos.1 + minY.abs()) as usize][(pos.0 + minX.abs()) as usize] = '#';
            }
        }
        for y in (0..map.len()).rev() {
            for x in 0..map[y].len() {
                print!("{}", map[y][x]);
            }
            println!("");
        }
    }
}

fn follow(tail: &mut (i32, i32), head: (i32, i32)) {
    let mut dir = (0, 0);
    if (head.0 - tail.0).abs() > 1 {
        dir.0 = (head.0 - tail.0) / (head.0 - tail.0).abs();
        if (head.1 - tail.1).abs() >= 1 {
            dir.1 = (head.1 - tail.1) / (head.1 - tail.1).abs()
        }
    } else if (head.1 - tail.1).abs() > 1 {
        dir.1 = (head.1 - tail.1) / (head.1 - tail.1).abs();
        if (head.0 - tail.0).abs() >= 1 {
            dir.0 = (head.0 - tail.0) / (head.0 - tail.0).abs()
        }
    }
    tail.0 = tail.0 + dir.0;
    tail.1 = tail.1 + dir.1;
}

fn distance_between(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    ((head.0 - tail.0).abs(), (head.1 - tail.1).abs())
}
