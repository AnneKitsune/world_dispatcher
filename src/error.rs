use crate::*;

/// The types of errors a `System` can create.
#[derive(Debug)]
pub enum EcsError {
    /// A resource was not initialized in the `World` but the
    /// `System` tries to access it.
    ///
    /// Usually, this means no dispatcher was used and `World::initialize`
    /// was not called.
    NotInitialized,
    /// The requested resource is already borrowed.
    /// This error is created if the `System` tries to read a resource that
    /// has already been mutably borrowed.
    /// It can also happen when trying to mutably borrow a resource that is
    /// already being read.
    ///
    /// This error should not occur during normal use, as the dispatchers
    /// can recover easily.
    AlreadyBorrowed,
    /// The execution of the dispatcher failed and returned one or more errors.
    DispatcherExecutionFailed(Vec<EcsError>),
    /// This variant is for user-defined errors.
    /// To create an error of this type easily, use the `system_error!` macro.
    SystemError(Box<dyn Error + Send>),
}

/// The result of a `System`'s execution.
/// Returns Ok(()) on success, `EcsError` on failure.
/// To return a custom error from a system, use the
/// `system_error!` macro.
pub type SystemResult = Result<(), EcsError>;

/// Returns a custom error from a `System` during execution.
#[macro_export]
macro_rules! system_error {
    ($err:expr) => {
        return Err(EcsError::SystemError(Box::new($err)));
    };
}

#[cfg(test)]
mod tests {
    use crate::*;
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    fn system_return_custom_error() {
        #[derive(Debug)]
        struct CustomError;
        impl std::fmt::Display for CustomError {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "Custom error crash")
            }
        }
        impl Error for CustomError {}
        fn sys() -> SystemResult {
            system_error!(CustomError)
        }
        let mut s = sys.system();
        let result = s.run(&World::default());
        match result {
            Err(EcsError::SystemError(err)) => assert_eq!(err.to_string(), "Custom error crash"),
            _ => unreachable!(),
        }
    }
}
