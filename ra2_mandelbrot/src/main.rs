use num::complex::Complex;

fn calculate_mandelbrot(
    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>>
{
    let mut rows: Vec<_> = Vec::with_capacity(width);
    for img_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(height);
        for img_x in 0..width {
            let x_percent = (img_x as f64 ) / (width as f64);
            let y_percent = (img_y as f64) / (height as f64);
            let c_x = x_min + (x_max - x_min) * x_percent;
            let c_y = y_min + (y_max - y_min) * y_percent;
            let escaped_at = mandelbrot_atpoint(c_x, c_y, max_iters);
            row.push(escaped_at);
        }
        rows.push(row);
    }
    rows
}

fn mandelbrot_atpoint(
    c_x: f64,
    c_y: f64,
    max_iters: usize,
) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex::new(c_x, c_y);

    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iters
}

fn render_mandelbrot(escaped_vals: Vec<Vec<usize>>)
{
    for row in escaped_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => '🟦',
                3..=5 => '🆓',
                6..=10 => '🌍',
                11..=30 => '🔴',
                31..=100 => '🟡',
                101..=200 => '👽',
                201..=400 => '🐧',
                401..=700 => '🐼',
                _ => '🆒',

            };
            line.push(val);
        }
        println!("{}", line);
    }
}

fn main() {
    let mandelbrot = calculate_mandelbrot(1000, -2.0, 2.0, -2.0, 2.0, 120, 100);
    render_mandelbrot(mandelbrot);
}
