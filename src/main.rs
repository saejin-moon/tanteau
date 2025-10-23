use tanteau::*;

fn main() {
    // basic math
    let a = 1.;
    let b: f32 = 2.;
    println!("{}", min(a, b));
    println!("{}", b.sin());
    // random
    let mut rng = Rng::new();
    rng.set_min(4.).set_max(5.);
    println!("{}", rng.next());
    // color
    let mut color = Color::new();
    color.red(24).green(128).blue(127);
    println!("{}", color.get_hex());
}
