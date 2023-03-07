use std::path::Path;

use directories::ProjectDirs;

const QLF_NAME: &str = "org";
const ORG_NAME: &str = "mediawhaler";
const APP_NAME: &str = "Media Whaler";

static SYS_CONFIG: &str = "/etc/mediawhaler/";

pub struct Dirs {
    dirs: ProjectDirs,
}

impl Dirs {
    pub fn new() -> Option<Self> {
        Some(Self {
            dirs: ProjectDirs::from(QLF_NAME, ORG_NAME, APP_NAME)?,
        })
    }

    pub fn sys_config_path() -> Option<&'static Path> {
        let sys_path = Path::new(SYS_CONFIG);
        sys_path.exists().then_some(sys_path)
    }

    pub fn config(&self) -> &Path {
        self.dirs.config_dir()
    }

    pub fn cache(&self) -> &Path {
        self.dirs.cache_dir()
    }

    pub fn data(&self) -> &Path {
        self.dirs.data_dir()
    }

    pub fn state(&self) -> Option<&Path> {
        self.dirs.state_dir()
    }
}
