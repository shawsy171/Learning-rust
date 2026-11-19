//! Exercise 1: temperature conversion. Write the code AND the tests.

/// Convert Celsius to Fahrenheit. F = C * 9/5 + 32
pub fn c_to_f(celsius: f64) -> f64 {
    todo!("your code here")
}

/// Classify a Celsius temperature:
/// below 0.0 => "freezing", 0.0 up to (not incl.) 25.0 => "mild", 25.0 and above => "hot"
pub fn describe(celsius: f64) -> &'static str {
    todo!("your code here")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Write your tests here. Aim for: normal cases, boundaries (0.0, 25.0),
    // and think about how to compare floats safely.
}
