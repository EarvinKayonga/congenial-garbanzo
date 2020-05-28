use failure::{Error};

fn main() -> Result<(), Error> {
    tonic_build::configure()
    .compile(
        &["../spec/echo.proto"],
        &["../spec/googleapis", "../spec"],
    )?;

    Ok(())
}