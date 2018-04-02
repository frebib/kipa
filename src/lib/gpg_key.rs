//! Manages GPG keys using GPGME.

use error::*;
use key::Key;

use gpgme;

/// Provide wrapped GPGME functionality.
pub struct GpgKeyHandler {
    context: gpgme::Context
}

impl GpgKeyHandler {
    /// Create a new handler. Creates a new GPGME context.
    pub fn new() -> Result<Self> {
        let context = gpgme::Context::from_protocol(gpgme::Protocol::OpenPgp)
            .chain_err(|| "Error on creating GPGME context")?;
        trace!("Created GPG key handler");
        Ok(GpgKeyHandler {
            context: context
        })
    }

    /// Get the key for a key ID string. The string must be eight characters
    /// long.
    pub fn get_key(&mut self, key_id: String) -> Result<Key> {
        trace!("Requested key ID: {}", key_id);

        let key = self.context.find_key(key_id)
            .chain_err(|| "Error on finding key")?;
        let mut buffer = Vec::new();
        self.context.export_keys(
                &[key], gpgme::ExportMode::empty(), &mut buffer)
            .chain_err(|| "Error on exporting key")?;

        Ok(Key::new(buffer))
    }
}

