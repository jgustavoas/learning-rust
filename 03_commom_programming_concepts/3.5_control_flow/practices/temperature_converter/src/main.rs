fn main() {
    println!("Hello, world!");
    temperature_converter(100.0, 'C');
}

fn temperature_converter(t: f32, from_scale: char) {
    let mut t_converted: f32 = 0.0;
    let mut other_scale = "";

    if from_scale == 'F' {
        t_converted = (t - 32.0) * 0.5556;
        other_scale = "C";
    }
    if from_scale == 'C' {
        t_converted = (t * 1.8) + 32.0;
        other_scale = "F";
    }

    println!("{t}°{from_scale} equals {t_converted}°{other_scale}");
}
