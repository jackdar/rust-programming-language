// Chapter 6.3: Concise control flow with if let
// Listing 6-6: A match that only cares about executing code when the value is Some
fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // The following code behaves in the same manner as above
    let config_max2 = Some(3u8);
    if let Some(max) = config_max2 {
        println!("The maximum is configured to be {max}");
    }
}
