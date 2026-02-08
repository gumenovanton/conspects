ANYHOW
// used when i do not check error type
// when i just log my errors

ANYHOW::RESULT
use anyhow::Result;

// basic anyhow type is Result
fn read_file(path: &str) -> Result<String> { // should set only value type or ()
    let content = std::fs::read_to_string(path)?; // error will be converted to anyhow error
    Ok(content)
}

ANYHOW::ERROR
use anyhow::Error;

// can contain eny error
let io_error: Error = std::io::Error::new(
    std::io::ErrorKind::NotFound,
    "file not found"
).into();

// can convert any &str to Error
let string_error: Error = "something went wrong".into();

BALL!
// macro that returns an error immediately
use anyhow::{bail, Result};

fn validate_age(age: u32) -> Result<()> {
    if age > 150 {
        bail!("Возраст {} слишком большой", age);
    }

    if age < 18 {
        bail!("Возраст {} меньше 18 лет", age);
    }

    Ok(())
}
