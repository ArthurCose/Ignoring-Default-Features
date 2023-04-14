pub fn test() {
    #[cfg(feature = "a")]
    println!("feature \"a\" is enabled!");

    #[cfg(feature = "b")]
    println!("feature \"b\" is enabled!");
}
