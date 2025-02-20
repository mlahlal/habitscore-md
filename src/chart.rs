use crate::errors::Errcode;

use std::collections::HashMap;
use plotters::prelude::*;

pub fn draw_chart(data: &HashMap<String, i32>) -> Result<(), Errcode> {
    let root = BitMapBackend::new("chart.png", (1366, 720)).into_drawing_area();
    let _ = root.fill(&WHITE);

    let tmp: Vec<String> = data.keys().cloned().collect();
    let mut x_labels: Vec<&str> = tmp.iter().map(AsRef::as_ref).collect();
    x_labels.sort_by(|a, b| a.cmp(b));
    x_labels.sort_by(|a, b| a.split("/").last().unwrap().cmp(b.split("/").last().unwrap()));

    let mut chart = match ChartBuilder::on(&root)
        .caption("Total Tasks per Day", ("sans-serif", 50).into_font())
        .x_label_area_size(20)
        .y_label_area_size(10)
        .margin(10)
        .build_cartesian_2d(
            x_labels.as_slice(),
            0..*data.values().max().unwrap_or(&0)+2,
        ) {
            Ok(chart) => chart,
            Err(e) => {
                log::error!("Error configuring chart builder");
                return Err(Errcode::UnknownError(e.to_string()));
            }
    };

    if let Err(e) = chart
        .configure_mesh()
        .x_labels(30)
        .y_labels(30)
        .x_label_formatter(&|x| x.to_string())
        .y_label_formatter(&|y| y.to_string())
        .draw() {
            log::error!("Error configuring mesh");
            return Err(Errcode::UnknownError(e.to_string()));
    }

    let data_points: Vec<(&str, i32)> = data.iter().map(|(date, value)| (date.as_str(), *value)).collect();

    if let Err(e) = chart.draw_series(
        data_points.iter().map(|(date, value)| Circle::new((date, *value), 5, RED.filled())),
        //LineSeries::new(data_points.iter().map(|(date, value)| (date, *value)), RED)
    ) {
        log::error!("Error drawing series");
        return Err(Errcode::UnknownError(e.to_string()));
    }

    Ok(())
}
