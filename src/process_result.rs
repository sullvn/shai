use std::io::{stderr, Write};
use std::process::{ExitCode, Termination};

use crate::result::Result;

/// ProcessResult for human readable errors
///
/// `std::result::Result` [is harcoded][0]
/// to print out debug information via the
/// `Debug` trait when returned from the
/// `main` function.
///
/// `ProcessResult` re-implements this
/// behavior, but via the `Display` trait
/// instead. This allows clean separation
/// of debug formatting and human readable
/// errors returned from the process on
/// stderr.
///
/// [0]: https://doc.rust-lang.org/std/result/enum.Result.html#impl-Termination-for-Result%3CT%2C%20E%3E
///
pub struct ProcessResult<T>(Result<T>);

impl<T> From<Result<T>> for ProcessResult<T> {
    fn from(result: Result<T>) -> Self {
        ProcessResult(result)
    }
}

impl<T: Termination> Termination for ProcessResult<T> {
    fn report(self) -> ExitCode {
        match self {
            ProcessResult(Ok(val)) => val.report(),
            ProcessResult(Err(err)) => {
                // Ignore error if stderr is already closed.
                // A panic also wouldn't be useful, as the
                // resulting error message would run into the
                // same problem.
                let _ = stderr().write_all(err.to_string().as_bytes());
                ExitCode::FAILURE
            }
        }
    }
}
