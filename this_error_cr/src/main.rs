use thiserror::Error;

#[derive(Error, Debug)]
enum LoginError {
    #[error("database error")]
    DatabaseError(#[from]  SqError),

    PasswordExpired,

    UserNotFound,

    NetworkError,

    WrongPassword,
}

fn login(user: &str, password: &str) -> Result<String, LoginError> {
    let connection: Result<Connection, std::io::Error> = connected()?;
    let user_id = get_user_id(user)?;
    if try_password()
}

fn main() {
    println!("Hello, world!");
}
