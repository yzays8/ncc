#![deny(unsafe_code)]

use ncc::driver::Driver;

fn main() -> anyhow::Result<()> {
    let driver = Driver::new();
    driver.run()?;
    Ok(())
}
