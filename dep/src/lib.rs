#[cfg(feature = "xyz")]
pub fn xyz() {
    println!("xyz enabled");
}


#[cfg(not(feature = "xyz"))]
pub fn xyz() {
    println!("xyz disabled");
}
