use crate::cli::Args;
use crate::errors::Errcode;

use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader};
use plotters::prelude::*;

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
            Err(e) => return Err(Errcode::UnknownError(e.to_string())),
        };

        let reader = BufReader::new(file);

        let mut tasks = false;
        let mut not_done: Vec<String> = Vec::new();
        let mut points = 0;
        let mut day = String::new();
        let lvl;

        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(e) => return Err(Errcode::UnknownError(e.to_string())),
            };

            match self.analyze(line, &mut tasks, &mut not_done, &mut points, &mut day) {
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

        self.chart()?;

        Ok(())
    }

    pub fn analyze <'a> (
        &mut self,
        line: String,
        tasks: &mut bool,
        not_done: &mut Vec<String>,
        points: &mut i32,
        day: &mut String
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

            if tmp_pt < *points { day.clear() }
        } else if !day.is_empty() && *tasks {
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
            *day = line.clone();
        }



        Ok(())
    }

    pub fn chart(&mut self) -> Result<(), Errcode> {
        const FILE_NAME: &str = "/home/mouad/Documents/dev/habitscore-md/sample.png";
        let root_area = BitMapBackend::new(FILE_NAME, (1280, 768)).into_drawing_area();

        if let Err(e) = root_area.fill(&WHITE) {
            return Err(Errcode::UnknownError(e.to_string()));
        }

        let root_area = match root_area.titled("Image Title", ("sans-serif", 60)) {
            Ok(x) => x,
            Err(e) => return Err(Errcode::UnknownError(e.to_string())),
        };

        let (upper, lower) = root_area.split_vertically(512);
        let x_axis = (-3.4f32..3.4).step(0.1);
        let mut cc = match ChartBuilder::on(&upper)
            .margin(5)
            .set_all_label_area_size(50)
            .caption("NxN M3x3 Polinomio Caratteristico", ("sans-serif", 40))
            .build_cartesian_2d(-3.4f32..3.4, -1.2f32..1.2f32) {
                Ok(x) => x,
                Err(e) => return Err(Errcode::UnknownError(e.to_string())),
        };

        if let Err(e) = cc.configure_mesh()
            .x_labels(20)
            .y_labels(10)
            .disable_mesh()
            .x_label_formatter(&|v| format!("{:.1}", v))
            .y_label_formatter(&|v| format!("{:.1}", v))
            .draw() {
                return Err(Errcode::UnknownError(e.to_string()));
        }

        cc.draw_series(LineSeries::new(x_axis.values().map(|x| (x, x.sin())), &RED)).unwrap()
          .label("Sine")
          .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

        if let Err(e) = cc.configure_series_labels().border_style(BLACK).draw() {
            return Err(Errcode::UnknownError(e.to_string()));
        }

        cc.draw_series(PointSeries::of_element(
            (-3.0f32..2.1f32).step(1.0).values().map(|x| (x, x.sin())),
            5,
            ShapeStyle::from(&RED).filled(),
            &|coord, size, style| {
                EmptyElement::at(coord)
                    + Circle::new((0, 0), size, style)
                    + Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
            },
        ));

        // To avoid the IO failure being ignored silently, we manually call the present function
        root_area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        println!("Result has been saved to {}", FILE_NAME);



        Ok(())
    }
}
