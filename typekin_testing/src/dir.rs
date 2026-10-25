use std::path::PathBuf;
use std::time::{
    SystemTime,
    UNIX_EPOCH,
};
use std::{
    env,
    fs,
};

pub struct FixtureDir {
    pub drop_rmdir: bool,
    pub path: PathBuf,
}

impl FixtureDir {
    pub fn mkdir_suffixed(suffix: &str) -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before the Unix epoch")
            .as_nanos();

        let path = env::temp_dir()
            .join(format!("typekin-{suffix}-{}-{unique}", std::process::id()));

        fs::create_dir_all(path.join("src"))
            .expect("unable to create directory");

        return Self {
            drop_rmdir: true,
            path,
        };
    }
}

impl Drop for FixtureDir {
    fn drop(&mut self) {
        if self.drop_rmdir {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

#[must_use]
pub fn make_dir(prefix: &str) -> PathBuf {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    return env::temp_dir().join(format!(
        "typekin_{}__{}__{}",
        prefix,
        std::process::id(),
        now
    ));
}
