//! Small, dependency-free settings file store for desktop applications.
//!
//! Keeps filesystem I/O away from GPUI Render implementations. The payload
//! format belongs to each application; this module only reads and writes text.
use std::{
    env,
    ffi::OsString,
    fs::{self, OpenOptions},
    io::{self, ErrorKind, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Clone)]
pub struct SettingsStore {
    path: PathBuf,
}

impl SettingsStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Resolve the Windows per-user settings path. The override supports
    /// portable builds and integration tests without modifying user settings.
    pub fn user_path(app_id: &str, config_dir_variable: &str) -> Option<PathBuf> {
        if app_id.is_empty()
            || app_id == "."
            || app_id == ".."
            || app_id.chars().any(|ch| matches!(ch, '/' | '\\' | ':' | '*' | '?'))
        {
            return None;
        }
        if let Some(dir) = env::var_os(config_dir_variable).filter(|dir| !dir.is_empty()) {
            return Some(PathBuf::from(dir).join("settings.conf"));
        }
        env::var_os("LOCALAPPDATA")
            .filter(|dir| !dir.is_empty())
            .map(|dir| PathBuf::from(dir).join(app_id).join("settings.conf"))
    }

    /// Missing settings are a normal first-run state, not an error.
    pub fn read(&self) -> io::Result<Option<String>> {
        match fs::read_to_string(&self.path) {
            Ok(contents) => Ok(Some(contents)),
            Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
            Err(error) => Err(error),
        }
    }

    /// Write a sibling temporary file and replace the destination with a
    /// single rename. A failed write never partially truncates an old file.
    pub fn write(&self, contents: &str) -> io::Result<()> {
        let parent = self.path.parent().ok_or_else(|| {
            io::Error::new(ErrorKind::InvalidInput, "settings path has no parent")
        })?;
        fs::create_dir_all(parent)?;
        let basename = self.path.file_name().ok_or_else(|| {
            io::Error::new(ErrorKind::InvalidInput, "settings path has no file name")
        })?;
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();

        for attempt in 0..8 {
            let mut name = OsString::from(basename);
            name.push(format!(".{}.{}.{}.tmp", std::process::id(), nonce, attempt));
            let temporary = parent.join(name);
            let file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temporary);
            let mut file = match file {
                Ok(file) => file,
                Err(error) if error.kind() == ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error),
            };
            let written = file
                .write_all(contents.as_bytes())
                .and_then(|_| file.sync_all());
            drop(file);
            let result = written.and_then(|_| fs::rename(&temporary, &self.path));
            if result.is_err() {
                let _ = fs::remove_file(&temporary);
            }
            return result;
        }
        Err(io::Error::new(
            ErrorKind::AlreadyExists,
            "could not allocate a unique temporary settings file",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absent_file_is_not_an_error() {
        let store = SettingsStore::new(std::env::temp_dir().join(format!(
            "argus-ui-nonexistent-{}-{}.conf",
            std::process::id(),
            9371
        )));
        assert_eq!(store.read().unwrap(), None);
    }

    #[test]
    fn saving_twice_replaces_the_previous_complete_file() {
        let dir = std::env::temp_dir().join(format!(
            "argus-ui-store-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));
        let store = SettingsStore::new(dir.join("nested").join("settings.conf"));
        store.write("version=1\ntheme=light\n").unwrap();
        assert_eq!(
            store.read().unwrap().as_deref(),
            Some("version=1\ntheme=light\n")
        );
        store.write("version=1\ntheme=dark\n").unwrap();
        assert_eq!(
            store.read().unwrap().as_deref(),
            Some("version=1\ntheme=dark\n")
        );
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn application_identifier_cannot_escape_its_settings_directory() {
        assert!(SettingsStore::user_path("..", "ARGUS_UI_CONFIG_DIR").is_none());
        assert!(SettingsStore::user_path("../other", "ARGUS_UI_CONFIG_DIR").is_none());
        assert!(SettingsStore::user_path("some\\\\other", "ARGUS_UI_CONFIG_DIR").is_none());
        assert!(SettingsStore::user_path("my:app", "ARGUS_UI_CONFIG_DIR").is_none());
    }

}
