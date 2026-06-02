use crate::runtime::docker::{Distro, exec};
use anyhow::Result;

pub fn create_user(container: &str, distro: &Distro, username: &str) -> Result<()> {
    match distro {
        Distro::Ubuntu => {
            exec(
                container,
                &format!(
                    "id -u {} >/dev/null 2>&1 || \
                     useradd -m -s /bin/bash {}",
                    username, username
                ),
            )?;

            exec(&container, &format!("usermod -aG sudo {}", username))?;

            exec(
                &container,
                "echo '%sudo ALL=(ALL:ALL) NOPASSWD: ALL' > /etc/sudoers.d/lenv && chmod 440 /etc/sudoers.d/lenv",
            )?;
        }

        Distro::Arch => {
            exec(
                &container,
                r#"sed -i 's/^# %wheel ALL=(ALL:ALL) NOPASSWD: ALL/%wheel ALL=(ALL:ALL) NOPASSWD: ALL/' /etc/sudoers"#,
            )?;
        }

        Distro::Alpine => {
            exec(
                container,
                &format!(
                    "id -u {} >/dev/null 2>&1 || \
                     adduser -D {}",
                    username, username
                ),
            )?;

            exec(&container, &format!("addgroup {} wheel", username))?;

            exec(
                &container,
                r#"echo '%wheel ALL=(ALL) NOPASSWD:ALL ALL' >> /etc/sudoers"#,
            )?;
        }
    }

    Ok(())
}
