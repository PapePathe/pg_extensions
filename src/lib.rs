use nanoid::nanoid;
use pgrx::prelude::*;
::pgrx::pg_module_magic!();

pub mod masked_email;

#[pg_extern]
fn hello_engagements() -> &'static str {
    "Hello, engagements"
}

#[pg_extern]
fn new_nano_id() -> &'static str {
    let id = nanoid!();
    Box::leak(id.into_boxed_str())
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_engagements() {
        assert_eq!("Hello, engagements", crate::hello_engagements());
    }

    #[pg_test]
    fn test_new_nano_id() {
        let result: &str = crate::new_nano_id();
        assert_eq!(21, result.len());
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
