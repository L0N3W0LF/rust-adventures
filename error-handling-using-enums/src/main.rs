// Rust's std::result type
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Rust's std::option type
enum Option<T> {
    Some(T),
    None,
}

fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    match b {
        0 => Result::Err("divide by zero"),
        _ => Result::Ok(a / b),
    }
}

fn find_item(nr: i32) -> Option<&'static str> {
    match nr {
        0 => Option::Some("Rust"),
        _ => Option::None,
    }
}

fn main() {
    match divide(1, 1) {
        Result::Ok(result) => println!("1 / 1 = {}", result),
        Result::Err(reason) => println!("Error: {}", reason),
    }

    match divide(1, 0) {
        Result::Ok(result) => println!("1 / 0 = {}", result),
        Result::Err(reason) => println!("Error: {}", reason),
    }

    match find_item(0) {
        Option::Some(result) => println!("Item: {}", result),
        Option::None => println!("No item was found"),
    }

    match find_item(1) {
        Option::Some(result) => println!("Item: {}", result),
        Option::None => println!("No item was found"),
    }
}
