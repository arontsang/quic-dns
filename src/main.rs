fn main() -> Result<(), Box<std::io::Error>> {
    let runtime = compio::runtime::Runtime::new()?;
    runtime.block_on(main_async());
    Ok(())
}

async fn main_async() {
    println!("Hello, world!");
}
