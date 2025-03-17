use crate::errors::Errcode;

use std::collections::HashMap;
use plotters::prelude::*;

pub fn draw_chart(data: &HashMap<String, i32>) -> Result<(), Errcode> {
    let root = BitMapBackend::new("chart.png", (1920, 720)).into_drawing_area();
    let _ = root.fill(&WHITE);
    let mut data_points: Vec<(&str, i32)> = data.iter().map(|(date, value)| (date.as_str(), *value)).collect();

    data_points.sort_by(|a, b| a.0.cmp(b.0));
    data_points.sort_by(|a, b| a.0.split("/").last().unwrap().cmp(b.0.split("/").last().unwrap()));

    let mut chart = match ChartBuilder::on(&root)
        .caption("Total Tasks per Day", ("sans-serif", 50).into_font())
        .x_label_area_size(20)
        .y_label_area_size(10)
        .margin(10)
        .build_cartesian_2d(
            0..data_points.len()-1,
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
        .x_labels(data_points.len()/2)
        .y_labels(30)
        .disable_x_mesh()
        .x_label_formatter(&|x| data_points[*x].0.to_string())
        .y_label_formatter(&|y| y.to_string())
        .draw() {
            log::error!("Error configuring mesh");
            return Err(Errcode::UnknownError(e.to_string()));
    }

    if let Err(e) = chart.draw_series(
        data_points.iter().zip(0..data_points.len()).map(|tuple| Circle::new((tuple.1, tuple.0.1), 2, RED.filled())),
    ) {
        log::error!("Error drawing series");
        return Err(Errcode::UnknownError(e.to_string()));
    }

    if let Err(e) = chart.draw_series(
        LineSeries::new(data_points.iter().zip(0..data_points.len()).map(|tuple| (tuple.1, tuple.0.1)), RED),
    ) {
        log::error!("Error drawing series");
        return Err(Errcode::UnknownError(e.to_string()));
    }

    Ok(())
}
