use anyhow::{Result, bail};

pub fn validate_name(name: &str) -> Result<()> {
    if name.is_empty() {
        bail!("environment name cannot be empty")
    }

    if name.len() > 64 {
        bail!("environment name is too long");
    }

    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        bail!(
            "environment name may only contain letters, numbers, '-' and '_'"
        );
    }

    Ok(())
}
