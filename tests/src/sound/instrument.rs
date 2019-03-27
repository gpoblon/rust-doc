pub fn clarinet() -> String {
    println!("clarinet() called");
    String::from("clarinet")
}

pub fn pv_drum() {
    println!("pv_drum) called");
    crate::sound::ake();
}