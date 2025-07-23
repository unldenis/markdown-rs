fn main() -> Result<(), markdown::message::Message> {
    // Turn on debugging.
    // You can show it with `RUST_LOG=debug cargo run --features log --example lib`
    env_logger::init();

    // Turn markdown into FrankenUI.
    println!("{}", markdown::to_frankenui("## Hello, *FrankenUI*!"));

    Ok(())
}
