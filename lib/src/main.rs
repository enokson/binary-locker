use sha256::{digest, try_digest};
use std::fs::{read_dir, ReadDir, self};
use std::path::{Path, PathBuf, self};

#[derive(Debug)]
struct ProjectConfig {
    servers: Vec<String>,
    name: String,
    include: Vec<String>,
    exclude: Vec<String>
}

struct UserConfig {
    key: String,
}

struct LocalConfig {
    write_dir: String,
    read_dir: String,
    store: Option<String>
}

#[derive(Debug)]
struct FileInfo {
    path: PathBuf,
    name: String
}

fn getFileInfo(mut iter: ReadDir, mut file_info_list: Vec<FileInfo>) -> Result<Vec<FileInfo>, String> {
    let entry_result = match iter.next() {
        Some(entry_result) => entry_result,
        None => { return Ok(file_info_list) }
    };
    let entry = match entry_result {
        Ok(entry) => entry,
        Err(error) => return Err(error.to_string())
    };
    let metadata = match entry.metadata() {
        Ok(metadata) => metadata,
        Err(error) => return Err(error.to_string())
    };
    if metadata.is_file() {
        let name = {
            let mut hash = match try_digest(entry.path()) {
                Ok(hash) => hash,
                Err(error) => return Err(error.to_string())
            };
            hash.push_str(&format!("-{}", entry.file_name().to_str().unwrap()));
            hash
        };
        file_info_list.push(FileInfo {
            path: entry.path(),
            name: entry.file_name().to_string_lossy().to_string()
        });
        getFileInfo(iter, file_info_list)
    } else if metadata.is_dir() {
        let child_iter = match read_dir(entry.path()) {
            Ok(read_dir_object) => read_dir_object,
            Err(error) =>  return Err(error.to_string())
        };
        let list = match getFileInfo(child_iter, file_info_list) {
            Ok(list) => list,
            Err(error) => return Err(error)
        };
        getFileInfo(iter, list)
    } else {
        Err(format!("File type unsupported: '{:#?}'", entry.path()))
    }
}

fn copy_to_store(mut manifest: String, mut file_info_list: Vec<FileInfo>, store_path: &Path) -> Result<String, String> {
    let file_info = match file_info_list.pop() {
        Some(file_info) => file_info,
        None => return Ok(manifest)
    };
    manifest.push_str(&file_info.name);
    manifest.push(' ');
    manifest.push_str(file_info.path.as_path().to_str().unwrap());
    manifest.push('\n');
    let mut store_file_path = PathBuf::from(store_path);
    store_file_path.push(file_info.name);
    dbg!(&file_info.path);
    dbg!(store_file_path);
    dbg!(&manifest);
    if !file_info.path.exists() {
        return Err(format!("Could not find src file {}", file_info.path.to_str().unwrap()))
    }
    if let Err(error) = fs::copy(&file_info.path, store_path) {
        return Err(format!("Could not copy {} to store. {}", file_info.path.to_str().unwrap(), error))
    }
    copy_to_store(manifest, file_info_list, store_path)
}

fn commitToStore(project_config: &ProjectConfig, local_config: &LocalConfig) -> Result<(), String> {
    let write_dir_path = PathBuf::from(local_config.write_dir.clone());
    if !write_dir_path.exists() {
        return Err("Write Directory could not be found".to_string());
    }
    let store_dir_path = match &local_config.store {
        Some(store_dir_path) => PathBuf::from(store_dir_path.clone()),
        None => return Err("Store directory could not be found".to_string())
    };
    let mut manifest_text: String = String::new();
    let iter = match read_dir(write_dir_path) {
        Ok(read_dir_object) => read_dir_object,
        Err(error) =>  return Err(error.to_string())
    };
    let fileInfo = match getFileInfo(iter, vec![]) {
        Ok(file_info) => file_info,
        Err(error) => return Err(error)
    };
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;
    use std::env;
    use std::fs;
    use home::home_dir;
    #[test]
    fn get_file_info () -> Result<(), String> {
        // let write_dir_path = "./test/writable/";
        let mut writable_dir_path = home_dir().unwrap();
        writable_dir_path.push("Documents");
        writable_dir_path.push("binary-locker-test");
        writable_dir_path.push("writable");
        let iter = match read_dir(&writable_dir_path) {
            Ok(read_dir_object) => read_dir_object,
            Err(error) =>  return Err(format!("could not read writable directory. {}", error))
        };
        let file_info_list = match getFileInfo(iter, vec![]) {
            Ok(file_info) => file_info,
            Err(error) => return Err(format!("Could not get file info. {}", error))
        };
        for info in file_info_list {
            dbg!(info);
        }
        Ok(())
    }

    #[test]
    fn returns_manifest() -> Result<(), String> {
        // dbg!(cwd);
        let writable_dir_path = {
            let mut p = home_dir().unwrap();
            p.push("Documents");
            p.push("binary-locker-test");
            p.push("writable");
            p
        };
        let store_dir_path = {
            let mut p = home_dir().unwrap();
            p.push("Documents");
            p.push("binary-locker-test");
            p.push("store");
            p
        };
        // let store_dir = match TempDir::new_in(".","temp-store") {
        //     Ok(temp_dir) => temp_dir,
        //     Err(error) => return Err(format!("Could not create temp path. {}", error))
        // };
        // let store_dir_path = store_dir.path();
        let iter = match read_dir(writable_dir_path) {
            Ok(read_dir_object) => read_dir_object,
            Err(error) =>  return Err(format!("Could not read writable directory. {}", error))
        };
        let mut file_info_list = match getFileInfo(iter, vec![]) {
            Ok(file_info) => file_info,
            Err(error) => return Err(error)
        };
        file_info_list.reverse();
        
        let manifest = match copy_to_store(String::new(), file_info_list, &store_dir_path) {
            Ok(manifest) => manifest,
            Err(error) => return Err(error)
        };
        dbg!(manifest);
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
