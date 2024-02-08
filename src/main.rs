use plotters::prelude::*;
use std::f32::consts::PI;

fn main() {
    let result = draw_quadratic_function();
    let result2 = draw_arc();
}

fn draw_quadratic_function() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("quadratic.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Quadratic Function", ("sans-serif", 30))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-10f32..10f32, -10f32..10f32)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        (-100..=100).map(|x| (x as f32 / 10.0, x as f32 / 10.0 * x as f32 / 10.0)),
        &RED,
    ))?;

    //x軸
    chart.draw_series(LineSeries::new(
        (-100..=100).map(|x| (x as f32 / 10.0, 0.0 * x as f32 / 10.0)),
        &BLUE,
    ))?;

    //y軸
    chart.draw_series(LineSeries::new(
        (-100..=100).map(|y| (0.0 * y as f32 / 10.0, y as f32 / 10.0)),
        &BLUE,
    ))?;

    Ok(())
}

fn draw_arc() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("arc.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-150f32..150f32, -150f32..150f32)?;

    //円弧
    chart.draw_series(LineSeries::new(
        (0..=1000).map(|x| {
            (
                100.0 * (((x as f32) / 2000.0 * PI).cos()),
                100.0 * (((x as f32) / 2000.0 * PI).sin()),
            )
        }),
        &RED,
    ))?;

    //y軸の線
    chart.draw_series(LineSeries::new(
        (0..=1000).map(|x| ((x as f32) * 0.0, 100.0 * ((x as f32) / 1000.0))),
        &RED,
    ))?;

    //x軸の線
    chart.draw_series(LineSeries::new(
        (0..=1000).map(|x| (100.0 * (x as f32) / 1000.0, (x as f32) * 0.0)),
        &RED,
    ))?;

    Ok(())
}

fn alert_humidity(temp: i32) -> String {
    if temp <= 45 {
        "湿度をあげてください".to_string()
    } else {
        "充分な湿度です".to_string()
    }
}

//AI code
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_humidity_low_temp() {
        // 低い温度の場合
        let result = alert_humidity(40);
        assert_eq!(result, "湿度をあげてください");
    }

    #[test]
    fn test_alert_humidity_high_temp() {
        // 高い温度の場合
        let result = alert_humidity(50);
        assert_eq!(result, "充分な湿度です");
    }
}
