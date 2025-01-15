    use std::fs;

fn main() {
    let contents = fs::read_to_string("/path/to/file")
        .expect("Should have been able to read the file");
    let contents: Vec<&str> = contents.split('\n').collect();
    let mut lines = vec![];
    let mut day = String::new();
    let mut tasks: bool = false;
    let mut points = 0;

    for line in &contents {
        let tmp: String = line.to_string();

        if tasks && tmp.chars().count() >= 2 && &tmp[..3] == "- [" {
            match &tmp[6..7] {
                "H" => {
                    points = points + ( if &tmp[3..4] == "X" {10} else {-15} );
                },
                "M" => {
                    points = points + ( if &tmp[3..4] == "X" {5} else {-3} );
                },
                "L" => {
                    points = points + ( if &tmp[3..4] == "X" {1} else {0} );
                },
                _ => println!("Stop The Cap"),
            }
            continue;
        } else {
            tasks = false;
        }

        // Soluzione non funzionante, RIVEDERE
        if tmp.chars().count() >= 6 && &tmp[..7] == "Punti:" {
            let tmp_pt = tmp.chars().skip_while(|c| !c.is_digit(10)).collect::<String>();
            points = points + i32::from_str_radix(&tmp_pt[0..1], 10);
        }

        if tmp.chars().count() >= 4 && &tmp[0..4] == "####" {
            tasks = true;
            day = tmp.clone();
        }

        lines.push(format!("..{}..", tmp));
    }

    println!("{}", points);
}
