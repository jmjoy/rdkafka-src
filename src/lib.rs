use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
    thread::available_parallelism,
};

pub fn source_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("librdkafka")
}

#[derive(Debug)]
pub struct Artifacts {
    pub install_dir: PathBuf,
    pub include_dir: PathBuf,
    pub lib_dir: PathBuf,
}

#[derive(Debug)]
pub struct Build {
    build_dir: PathBuf,
    install_dir: PathBuf,
    source_dir: PathBuf,
    version: String,
    num_jobs: Option<usize>,
}

impl Build {
    pub fn new(out_dir: impl Into<PathBuf>, version: impl Into<String>) -> Self {
        let out_dir = out_dir.into();
        let version = version.into();

        Self {
            build_dir: out_dir.join("librdkafka-build").join(&version),
            install_dir: out_dir.join("librdkafka").join(&version),
            source_dir: source_dir(),
            version,
            num_jobs: None,
        }
    }

    pub fn num_jobs(mut self, num_jobs: usize) -> Self {
        self.num_jobs = Some(num_jobs);
        self
    }

    pub fn build(self) -> Result<Artifacts, Box<dyn Error>> {
        self.clean_up()?;

        copy_all(&self.source_dir, &self.build_dir)?;

        self.checkout_version()?;
        self.configure()?;
        self.make()?;
        self.make_install()?;

        fs::remove_dir_all(&self.build_dir)?;

        Ok(Artifacts {
            install_dir: self.install_dir.clone(),
            include_dir: self.install_dir.join("include"),
            lib_dir: self.install_dir.join("lib"),
        })
    }

    fn clean_up(&self) -> Result<(), Box<dyn Error>> {
        if fs::exists(&self.install_dir)? {
            fs::remove_dir_all(&self.install_dir)?;
        }
        fs::create_dir_all(&self.install_dir)?;

        if fs::exists(&self.build_dir)? {
            fs::remove_dir_all(&self.build_dir)?;
        }
        fs::create_dir_all(&self.build_dir)?;

        Ok(())
    }

    fn checkout_version(&self) -> Result<(), Box<dyn Error>> {
        let version = format!("v{}.*", self.version);
        let tags = execute_command(&["git", "tag", "-l", &version], &self.build_dir)?;

        // Get the maximum revision number.
        let ver = tags
            .split_whitespace()
            .filter_map(|tag| tag.splitn(3, '.').nth(2))
            .filter_map(|ver| ver.parse::<usize>().ok())
            .max();
        let version = if let Some(ver) = ver {
            format!("v{}.{}", self.version, ver)
        } else {
            return Err("version not found".into());
        };

        execute_command(&["git", "checkout", &version], &self.build_dir)?;
        Ok(())
    }

    fn configure(&self) -> Result<(), Box<dyn Error>> {
        execute_command(
            &[
                "./configure",
                "--prefix",
                self.install_dir.display().to_string().as_str(),
                "--disable-zlib",
                "--disable-zstd",
                "--disable-ssl",
                "--disable-gssapi",
                "--disable-sasl",
                "--disable-curl",
                "--disable-lz4-ext",
                "--disable-lz4",
                "--disable-regex-ext",
            ],
            &self.build_dir,
        )?;
        Ok(())
    }

    fn make(&self) -> Result<(), Box<dyn Error>> {
        let num_jobs = if let Some(num_jobs) = self.num_jobs {
            num_jobs
        } else {
            let num_jobs = available_parallelism()?.get();
            if num_jobs >= 3 { num_jobs - 2 } else { 1 }
        };
        execute_command(&["make", "-j", &*num_jobs.to_string()], &self.build_dir)?;
        Ok(())
    }

    fn make_install(&self) -> Result<(), Box<dyn Error>> {
        execute_command(&["make", "install"], &self.build_dir)?;
        Ok(())
    }
}

fn copy_all(src: &Path, dst: &Path) -> Result<(), Box<dyn Error>> {
    if src.is_file() {
        fs::copy(src, dst)?;
    } else if src.is_dir() {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());
            if src_path.is_dir() {
                copy_all(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }
    } else {
        return Err(format!("{} is not file or directory", src.display()).into());
    }
    Ok(())
}
fn execute_command(cmd: &[&str], current_dir: impl AsRef<Path>) -> Result<String, Box<dyn Error>> {
    let [program, args @ ..] = cmd else {
        return Err("cmd is empty".to_string().into());
    };

    println!(
        "execute command in `{}`: `{}`",
        current_dir.as_ref().display(),
        cmd.join(" ")
    );

    let output = Command::new(program)
        .args(args)
        .current_dir(current_dir.as_ref())
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    println!("{}", stdout);
    eprintln!("{}", String::from_utf8_lossy(&output.stderr));

    Ok(stdout.to_string())
}
