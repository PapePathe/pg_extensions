use pgrx::prelude::*;

#[pg_extern]
pub fn mask_email(email: &str) -> String {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return String::new();
    }

    let local_part = parts[0];
    let domain = parts[1];

    if local_part.is_empty() || domain.is_empty() {
        return String::new();
    }

    let masked_local = format!("{}***", &local_part.chars().next().unwrap());
    format!("{}@{}", masked_local, domain)
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {

    #[test]
    fn test_mask_email() {
        // Define test cases
        let test_cases = vec![
            // Valid cases
            ("user@example.com", "u***@example.com"), // Normal case
            ("a@example.com", "a***@example.com"),    // Single-character local part
            ("alice.bob@domain.org", "a***@domain.org"), // Local part with dot
            ("ALICE.BOB@DOMAIN.ORG", "A***@DOMAIN.ORG"), // Local part with dot
            // Invalid cases
            ("@example.com", ""), // Missing local part
            ("invalidemail", ""), // Missing @ symbol
            ("", ""),             // Empty string
            ("no-domain@", ""),   // Missing domain part
            // Edge invalid cases
            ("user@-domain.com", "u***@-domain.com"), // Invalid domain starting with a dash
            ("user@domain..com", "u***@domain..com"), // Invalid consecutive dots in domain
            ("user@.domain.com", "u***@.domain.com"),
        ];

        // Run tests
        for (input, expected) in test_cases {
            let output = crate::masked_email::mask_email(input);
            assert_eq!(
                output, expected,
                "Failed for input: {} output: {}",
                input, output
            );
        }
    }
}
