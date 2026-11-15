pub fn get() -> &'static [&'static str] {
    &["cargo", "run", "--", "--addr", "0.0.0.0:6969"]
}
