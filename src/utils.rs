use std::time::Instant;

pub fn print_elapsed(start: &Instant, label: &str) {
    let millis = start.elapsed().as_millis();
    let seconds = millis / 1000;
    let (h, m, s) = (seconds / 3600, (seconds % 3600) / 60, seconds % 60);
    println!("{}: {:02}:{:02}:{:02}.{:03}", label, h, m, s, millis % 1000);
}