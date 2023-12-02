// main.rs
// load module from dec_1/lib.rs
mod dec_1;


fn main() -> std::io::Result<()> {
    dec_1::run()?;
    Ok(())
}