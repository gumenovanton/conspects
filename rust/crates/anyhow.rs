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

ENSHURE!
// check the bool expression with error return
// short bail!
use anyhow::ensure;

fn transfer_money(amount: f64, balance: f64) -> Result<()> {
    ensure!(amount > 0.0, "Summ should be positive");
    ensure!(balance >= amount, "Not enough money");

    Ok(())
}

ANYHOW!
// creates an anyhow error on the from string
use anyhow::anyhow;

// like this
fn find_user(id: u32) -> Result<User> {
    match database::get_user(id) {
        Some(user) => Ok(user),
        None => Err(anyhow!("User with ID {} is not found", id)),
    }
}

// or like this
let error: anyhow::Error = anyhow!("Error: code {}", 404);

CONTEXT
// add arror message to error messages chain
use anyhow::{Context, Result};

fn step1() -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!("DB error"))
}

fn step2() -> Result<()> {
    step1().context("Step 2 is crashed")?;
    Ok(())
}

fn main() {
    if let Err(e) = step2() {
        println!("Error: {}", e);
        // Error: "Step 2 is crashed"

        println!("Chain:");
        for (i, cause) in e.chain().enumerate() {
            println!("  {}: {}", i, cause);
        }
        // Chain:
        // 0: Step 2 is crashed
        // 1: DB Error
    }
}


WITH_CONTEXT
// lazy version of context
// creates error message only when error happens
use anyhow::{Context, Result};

fn process_file(filename: &str) -> Result<String> {
    // usefull when string creation is expensive
    std::fs::read_to_string(filename)
        .with_context(|| format!("File reading error {}", filename))?;

    // or for calculanions
    let result = expensive_operation()
        .with_context(|| {
            let count = get_attempt_count();
            format!("Operation failed after {} attemts", count)
        })?;

    Ok(result)
}

WORK WITH OPTIONS
// converts Option to an error
use anyhow::Context;

fn find_user(name: &str) -> anyhow::Result<User> {
    let user = users.iter()
        .find(|u| u.name == name) // returns an Option
        .context(format!("User '{}' not found", name))?; // convert to an error

    Ok(user.clone())
}

// work with any Option<T>
let value: Option<i32> = Some(42);
let result: anyhow::Result<i32> = value.context("Value do not exists");

DOWNCAST, DOWNCAST_REF
// attempt to get error type
// downcast - for ownership
// downcast_ref - for borrowing
use anyhow::Error;
use std::io;

fn handle_error(error: Error) {
    // try to get concrete error
    if let Some(io_error) = error.downcast_ref::<std::io::Error>() {
        println!("This is IO error: {}", io_error);
    } else if let Some(parse_error) = error.downcast_ref::<serde_json::Error>() {
        println!("This is JSON parsing error: {}", parse_error);
    } else {
        println!("Unknown error: {}", error);
    }
}

CHAIN
// an iterator on error chain
fn print_error_chain(error: &anyhow::Error) {
    println!("Error:");

    // go through all errors
    for (i, cause) in error.chain().enumerate() {
        if i == 0 {
            println!("  Base: {}", cause);
        } else {
            println!("  Reason {}: {}", i, cause);
        }
    }
}

// Error:
//   Base: Can't handle the file
//   Reason 1: Can't read the file
//   Reason 2: Permission denied

ROOT_CAUSE
// root cause of the error
let error = read_file("test.txt")
    .context("Cant read the file")
    .unwrap_err();

println!("Root cause: {}", error.root_cause());
