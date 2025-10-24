use tanto::*;

fn main() {
    // basic math
    println!("{}", min(1., 2.));
    // random
    let mut rng = Rng::new();
    rng.set_min(4.).set_max(5.);
    println!("{}", rng.next());
    // color
    let mut color = Color::new();
    color.red(24).green(128).blue(127);
    println!("{}", color.get_hex());
    // datetime
    let mut dt = DateTime::new();
    println!("{}:{}:{} {}/{}/{}", dt.hour(), dt.minute(), dt.second(), dt.month(), dt.day(), dt.year());
    dt.add(TimeUnit::Month, 2);
    println!("{}:{}:{} {}/{}/{}", dt.hour(), dt.minute(), dt.second(), dt.month(), dt.day(), dt.year());
}
