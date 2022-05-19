use bounce::color::Color;

fn main() {
    // output_test_image();

    let c1 = Color::new(1.0, 0.2, 0.4);
    let c2 = Color::new(0.9, 1.0, 0.1);
}

fn output_test_image() {
    let img_width = 256;
    let img_height = 256;

    println!("P3");
    println!("{} {}", img_width, img_height);
    println!("255");

    for j in (0..img_height).rev() {
        eprintln!("{} lines remaining", j);

        for i in 0..img_width {
            let c = Color::new(
                i as f64 / (img_width - 1) as f64,
                j as f64 / (img_height - 1) as f64,
                0.25,
            );

            let blend = Color::new(0.5, 0.7, 1.0);

            println!("{}", c);
        }
    }

    eprintln!("Done!");
}
