pub fn get_wilson_interval(positive: f64, total: f64, confidence: i64) -> (f64, f64) {
    if total == 0.0 {
        return (0.0, 1.0);
    }

    let phat = positive / total;

    // z is 1-alpha/2 percentile of a standard
    // normal distribution for error alpha=5%
    let z90 = 1.64;
    let z95 = 1.96;
    let z99 = 2.576;

    let z = match confidence {
        0 => z90,
        1 => z95,
        _ => z99,
    };

    let a = phat + z * z / (2.0 * total);
    let b = z * ((phat * (1.0 - phat) + z * z / (4.0 * total)) / total).sqrt();
    let c = 1.0 + z * z / total;

    ((a - b) / c, (a + b) / c)
}
