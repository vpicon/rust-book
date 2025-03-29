fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}

fn main() {
    let fahrenheit = 100.0;
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{fahrenheit} F is {celsius} C");
}
