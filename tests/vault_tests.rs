use rusty_vault::generator::{generate_password, GeneratorConfig};
use rusty_vault::vault::{Vault, VaultEntry};

#[test]
fn test_vault_crud_lifecycle() {
    let mut vault = Vault::new();
    let entry = VaultEntry::new(
        "GitHub",
        "pranav256",
        "pranav@example.com",
        "SecretPass123!",
    );
    vault.add_entry(entry.clone());
    assert_eq!(vault.entries.len(), 1);

    let search_results = vault.search("github");
    assert_eq!(search_results.len(), 1);
    assert_eq!(search_results[0].username, "pranav256");

    let deleted = vault.remove_entry(&entry.id);
    assert!(deleted);
    assert_eq!(vault.entries.len(), 0);
}

#[test]
fn test_password_generator_constraints() {
    let config = GeneratorConfig {
        length: 24,
        uppercase: true,
        lowercase: true,
        numbers: true,
        symbols: true,
        exclude_ambiguous: true,
    };
    let password = generate_password(&config);
    assert_eq!(password.len(), 24);
}
