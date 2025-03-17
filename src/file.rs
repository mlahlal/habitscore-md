use crate::cli::Args;
use crate::errors::Errcode;
use crate::chart::draw_chart;

use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub struct HabitFile {
    path: PathBuf,
    stat: bool,
}

impl HabitFile {
    pub fn new (args: Args) -> HabitFile {
        HabitFile {
            path: args.path,
            stat: args.stat,
        }
    }

    pub fn read (&mut self) -> Result<(), Errcode> {
        let file = match File::open(&self.path) {
            Ok(file) => file,
            Err(e) => return Err(Errcode::FileError(e.to_string())),
        };

        let reader = BufReader::new(file);

        let mut tasks = false;
        let mut not_done: Vec<String> = Vec::new();
        let mut tasks_per_day = HashMap::new();
        let mut points = 0;
        let mut day = String::new();
        let mut no_task = false;
        let lvl;

        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(e) => return Err(Errcode::FileError(e.to_string())),
            };

            match self.analyze(line, &mut tasks, &mut not_done, &mut points, &mut day, &mut no_task, &mut tasks_per_day) {
                Ok(_) => (),
                Err(e) => {
                    log::error!("Error while analizing file: \n\t{}", e);
                }
            }
        }

        if points < 0 { lvl = -1; }
        else { lvl = points/1000; }

        println!("points => {} \nlevel => {}", points, lvl);

        if self.stat {
            not_done.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

            struct NotDoneTask<'a> {
                task: &'a str,
                count: i32,
            }

            let mut stats: Vec<NotDoneTask> = Vec::new();

            for el in &not_done {
                let mut count = 0;
                for le in &not_done {
                    if le == el { count += 1; }
                }
                if !stats.iter().any(|i| i.task == *el) {
                    stats.push(NotDoneTask { task: el, count });
                } else {
                    let index = stats.iter().position(|r| r.task == *el).unwrap();
                    stats[index].count = count;
                }
            }

            println!("\n\nTask not done many times (>2)\n");
            for el in &stats {
                if el.count >= 2 {
                    println!("Task: {} \nCount: {}\n", el.task, el.count);
                }
            }
        }

        if self.stat {
            draw_chart(&tasks_per_day)?;
        }

        Ok(())
    }

    pub fn analyze <'a> (
        &mut self,
        line: String,
        tasks: &mut bool,
        not_done: &mut Vec<String>,
        points: &mut i32,
        day: &mut String,
        no_task: &mut bool,
        tasks_per_day: &mut HashMap<String, i32>
    ) -> Result<(), Errcode> {

        if *tasks && line.len() >= 2 && &line[..3] == "- [" {
            let tmp_pt = points.clone();
            let done = &line[3..4];
            let priority = &line[6..7];

            if self.stat && done == " " { not_done.push(line[10..].to_string()); }

            *points += match (done, priority) {
                ("X", "H") => 10,
                (_, "H") => -15,
                ("X", "M") => 5,
                (_, "M") => -3,
                ("X", "L") => 1,
                (_, "L") => 0,
                _ => {
                    return Err(Errcode::FormatInvalid(format!("Invalid format in line {}", line)));
                }
            };

            *tasks_per_day.entry(day.clone().replace("#### ", "")).or_insert(0) += 1;

            if tmp_pt < *points { *no_task = false; }
        } else if *no_task && *tasks {
            *points -= 10;
            *tasks = false;
        }

        if line.starts_with("Punti:") {
            let tmp_pt = line.chars()
                .skip_while(|c| !c.is_digit(10)  && *c != '-')
                .collect::<String>();
            let tmp_pt = tmp_pt.parse::<i32>().unwrap();
            *points += tmp_pt;
        }

        if line.starts_with("####") {
            *tasks = true;
            *no_task = true;
            *day = line.clone();
        }


        Ok(())
    }

}
