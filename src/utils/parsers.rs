/// Parse a positive f64 from a string, returning an error message if invalid.
pub fn parse_positive_f64(s: &str, field_name: &str) -> Result<f64, String> {
    match s.parse::<f64>() {
        Ok(v) if v > 0.0 => Ok(v),
        _ => Err(format!("请输入有效{}", field_name)),
    }
}
