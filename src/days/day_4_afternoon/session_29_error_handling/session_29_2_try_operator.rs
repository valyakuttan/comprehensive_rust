/// # 29.2 Try Operator
///
/// Runtime errors like connection-refused or file-not-found are
/// handled with the `Result` type, but matching this type on
/// every call can be cumbersome. The try-operator `?` is used to
/// return errors to the caller. It lets you turn the common
///
/// ```
///
/// match some_expression {
///     Ok(value) => value,
///     Err(err) => return Err(err),
/// }
///
/// ```
///
/// into the much simpler
///
/// ```
///
/// some_expression?
///
/// ```
///
/// We can use this to simplify our error handling code:
///
/// ```
///
/// use std::{fs, io};
/// use std::io::Read;
///
/// fn read_username(path: &str) -> Result<String, io::Error> {
///     let username_file_result = fs::File::open(path);
///     let mut username_file = match username_file_result {
///         Ok(file) => file,
///         Err(err) => return Err(err),
///     };
///
///     let mut username = String::new();
///     match username_file.read_to_string(&mut username) {
///         Ok(_) => Ok(username),
///         Err(err) => Err(err),
///     }
/// }
///
/// fn main_with_match() {
///     //fs::write("config.dat", "alice").unwrap();
///     let username = read_username("config.dat");
///     println!("username or error: {username:?}");
/// }
///
/// ```
/// The above code can be rewritten as
///
/// The `main` can return a `Result<(), E>` as long as it implements
/// `std::process:Termination`. In practice, this means that `E` implements
/// `Debug`. The executable will print the `Err` variant and return a nonzero
/// exit status on error.
///  
use std::fs;
use std::io::Error;
use std::io::Read;

fn read_username(path: &str) -> Result<String, Error> {
    let mut username = String::new();
    fs::File::open(path)?.read_to_string(&mut username)?;

    Ok(username)
}

pub fn main() -> Result<(), Error> {
    //fs::write("config.dat", "alice").unwrap();
    let username = read_username("config.dat")?;
    println!("username or error: {username}");

    Ok(())
}
