use keyring::{Entry, Result};

#[derive(Debug, Clone)]
pub struct Fort {
    name: String,
}

impl Fort {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// Store `application_key` for the given `key_id`.
    pub fn set_application_key(&self, id: &str, application_key: &str) -> Result<()> {
        let name = format_name(self.name(), id);
        let entry = Entry::new(&name, id.as_ref())?;
        entry.set_password(application_key)?;

        Ok(())
    }

    /// Get `application_key` for the given `key_id`.
    pub fn get_application_key(&self, id: &str) -> Result<String> {
        let name = format_name(self.name(), id);
        let entry = Entry::new(&name, id.as_ref())?;

        entry.get_password()
    }

    /// Remove `application_key` for the given `key_id`.
    pub fn delete_application_key(&self, id: &str) -> Result<()> {
        let name = format_name(self.name(), id);
        let entry = Entry::new(&name, id.as_ref())?;
        entry.delete_password()?;

        Ok(())
    }
}

fn format_name(name: &str, id: &str) -> String {
    format!("app.{}.connector.{}", name, id)
}
