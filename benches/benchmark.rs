use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_mask_email(c: &mut Criterion) {
    let domain_with5000_chars = "user@".to_owned() + &"a".repeat(5000) + ".com";
    //        &"a".repeat(1000) + "@example.com",              // Local part of 1000 characters
    //       "user@" + &"a".repeat(1000) + ".com",            // Domain part of 1000 characters
    //     &"a".repeat(5000) + "@example.com",              // Local part of 5000 characters
    //    &"a".repeat(10000) + "@example.com",             // Local part of 10,000 characters
    //   "user@" + &"a".repeat(10000) + ".com",           // Domain part of 10,000 characters

    let test_cases = vec![
        "user@example.com",              // Standard case
        "a@example.com",                 // Single-character local part
        "user+tag@domain.com",           // Plus addressing
        "UPPERCASE@EXAMPLE.COM",         // Uppercase email
        "user..extra@domain.com",        // Invalid consecutive dots in local part
        "verylongemailname@example.com", // Long local part
        "user@sub.domain.com",           // Subdomain
        "@example.com",                  // Missing local part
        "invalidemail",                  // Missing @ symbol
        "user@domain",                   // Missing TLD
        "user@domain.com",               // Simple valid case
        &domain_with5000_chars,
    ];

    c.bench_function("mask_email", |b| {
        b.iter(|| {
            for email in &test_cases {
                black_box(engagements::masked_email::mask_email(email));
            }
        });
    });
}

criterion_group!(benches, benchmark_mask_email);
criterion_main!(benches);
