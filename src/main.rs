use bounce::vec3::Vec3;

fn main() {
    let img_width = 256;
    let img_height = 256;

    println!("P3");
    println!("{} {}", img_width, img_height);
    println!("255");

    for j in (0..img_height).rev() {
        eprintln!("{} lines remaining", j);

        for i in 0..img_width {
            let r = i as f32 / (img_width - 1) as f32;
            let g = j as f32 / (img_height - 1) as f32;
            let b = 0.25;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("Done!");
}
