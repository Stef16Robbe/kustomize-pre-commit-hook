use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;

#[tokio::main]
async fn main() {
    let cwd = env::current_dir().unwrap();

    let args: Vec<String> = env::args().collect();

    let kustomize_paths: Vec<PathBuf> = args
        .iter()
        .skip(1) // thanks to pre-commit we can assume we get at least one `kustomization.yaml` file here
        .map(|p| Path::new(p).to_path_buf())
        .collect();

    let mut handles = Vec::new();

    for path in kustomize_paths {
        println!("Building: {}...", path.display());

        let path = path.clone();
        let cwd = cwd.clone();
        let handle = tokio::spawn(async move {
            let path = get_path_parent(&path).await;
            build(path, &cwd).await;
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }
}

async fn get_path_parent(path: &Path) -> &Path {
    path.parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."))
}

async fn build(path: &Path, cwd: &PathBuf) {
    let res = Command::new("kustomize")
        .current_dir(cwd)
        .arg("build")
        .arg(path)
        .output()
        .expect("error running kustomize build");

    if !res.status.success() {
        eprintln!(
            "error building {}: {:?}",
            path.display(),
            str::from_utf8(&res.stderr).unwrap_or("unknown error")
        );
    } else {
        // println!("{:?}", str::from_utf8(&res.stdout).unwrap());
        println!("Build {} success!", path.display());
    }
}
