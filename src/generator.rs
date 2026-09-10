use rand::seq::SliceRandom;

#[derive(Clone, Debug)]
pub struct GeneratorConfig {
    pub length: usize,
    pub uppercase: bool,
    pub lowercase: bool,
    pub numbers: bool,
    pub symbols: bool,
    pub exclude_ambiguous: bool,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            length: 16,
            uppercase: true,
            lowercase: true,
            numbers: true,
            symbols: true,
            exclude_ambiguous: false,
        }
    }
}

const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()_+-=[]{}|;:,.<>?";

const AMBIGUOUS: &[char] = &['I', 'l', '1', 'O', '0', 'o', '`', '~'];

/// Generate a cryptographically secure random password matching the configuration
pub fn generate_password(config: &GeneratorConfig) -> String {
    let mut charset = String::new();
    let mut guaranteed = Vec::new();
    let mut rng = rand::rngs::OsRng;

    let filter_ambiguous = |s: &str| -> Vec<char> {
        if config.exclude_ambiguous {
            s.chars().filter(|c| !AMBIGUOUS.contains(c)).collect()
        } else {
            s.chars().collect()
        }
    };

    if config.uppercase {
        let chars = filter_ambiguous(UPPER);
        if let Some(&c) = chars.choose(&mut rng) {
            guaranteed.push(c);
        }
        charset.extend(chars);
    }

    if config.lowercase {
        let chars = filter_ambiguous(LOWER);
        if let Some(&c) = chars.choose(&mut rng) {
            guaranteed.push(c);
        }
        charset.extend(chars);
    }

    if config.numbers {
        let chars = filter_ambiguous(NUMBERS);
        if let Some(&c) = chars.choose(&mut rng) {
            guaranteed.push(c);
        }
        charset.extend(chars);
    }

    if config.symbols {
        let chars = filter_ambiguous(SYMBOLS);
        if let Some(&c) = chars.choose(&mut rng) {
            guaranteed.push(c);
        }
        charset.extend(chars);
    }

    if charset.is_empty() {
        charset = LOWER.chars().collect();
    }

    let pool: Vec<char> = charset.chars().collect();
    let mut password = guaranteed;

    while password.len() < config.length.max(8) {
        if let Some(&c) = pool.choose(&mut rng) {
            password.push(c);
        }
    }

    password.shuffle(&mut rng);
    password.into_iter().collect()
}

/// Calculate estimated password entropy in bits
pub fn calculate_entropy(password: &str) -> f64 {
    let mut pool_size = 0;
    let mut has_lower = false;
    let mut has_upper = false;
    let mut has_digit = false;
    let mut has_symbol = false;

    for c in password.chars() {
        if c.is_ascii_lowercase() {
            has_lower = true;
        } else if c.is_ascii_uppercase() {
            has_upper = true;
        } else if c.is_ascii_digit() {
            has_digit = true;
        } else {
            has_symbol = true;
        }
    }

    if has_lower {
        pool_size += 26;
    }
    if has_upper {
        pool_size += 26;
    }
    if has_digit {
        pool_size += 10;
    }
    if has_symbol {
        pool_size += 32;
    }

    if pool_size == 0 {
        return 0.0;
    }

    password.len() as f64 * (pool_size as f64).log2()
}
