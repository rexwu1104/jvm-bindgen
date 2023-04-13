use std::{env::{self, current_dir}, io::{Cursor, Write, Read}, path::PathBuf, collections::HashMap, error::Error, fs::{OpenOptions, create_dir}};

use toml::Value;
use zip::ZipArchive;

#[derive(Debug)]
struct ZipFile {
    path: String,
    filename: String,
}

impl ZipFile {
    pub fn write(&self, parent: PathBuf, prefix: String, zip: &mut ZipArchive<&mut Cursor<&[u8]>>) -> Result<(), Box<dyn Error>> {
        let path = parent
            .join(self.path.trim_end_matches("\\"))
            .join(self.filename.clone());
        
        preprocess_dir(path.parent().unwrap().to_path_buf())?;
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(false)
            .open(path)?;

        let mut buf = String::new();
        let binding = [
            &prefix,
            "/",
            &self.path,
            &self.filename
        ].concat();
        let filename = binding
            .as_str();

        if filename.ends_with("module-info.java") {
            return Ok(())
        }
        
        let mut zip_file = zip.by_name(filename.replace(".rs", ".java").as_str()).unwrap();

        zip_file.read_to_string(&mut buf)?;
        buf = ["jvm_bindgen::parse_java!(", buf.as_str(), ")"].concat();
        file.write_all(buf.as_bytes())?;

        Ok(())
    }
}

#[derive(Debug)]
struct ZipDirectory {
    files: HashMap<String, Vec<ZipFile>>
}

fn preprocess_dir(path: PathBuf) -> Result<(), Box<dyn Error>> {
    if !path.is_dir() {
        preprocess_dir(path.parent().unwrap().to_path_buf())?;
        match create_dir(path) {
            _ => ()
        };
    }

    Ok(())
}

fn main() {
    let mut zip_cursor = Cursor::new(include_bytes!(concat!(env!("JAVA_HOME"), "lib\\", "src.zip")).as_slice());
    let mut zip = ZipArchive::new(&mut zip_cursor).unwrap();
    let mut files: HashMap<String, Vec<ZipFile>> = HashMap::new();
    for key in zip.file_names() {
        let prefix = key.split("/").next().unwrap();
        let mut filename: String = key.split("/").last().unwrap().to_string();
        let path = key.replace(&[prefix, "/"].concat(), "").replace(&filename, "");
        filename = filename.replace(".java", ".rs");
        println!("----------\n{prefix}\n{path}\n{filename}\n----------");
        if files.contains_key(prefix) {
            if let Some(v) = files.get_mut(prefix) {
                (*v).push(ZipFile {
                    path,
                    filename
                });
            }
        } else {
            files.insert(prefix.into(), vec![ZipFile {
                path,
                filename
            }]);
        }
    }

    let dir = ZipDirectory { files };
    let current_dir = current_dir().unwrap();
    let process_dir_path = current_dir.join(PathBuf::from("java_process"));
    for (prefix, files) in dir.files {
        files.iter().for_each(|file|
            file.write(process_dir_path.clone(), prefix.clone(), &mut zip).unwrap());
    }

    let path = env::var("JAVA_HOME").unwrap();
    let manifest: Value = toml::from_str(include_str!("Cargo.toml")).unwrap();

    let manifest_map = manifest.as_table().unwrap();
    let lib = manifest_map.get(stringify!(lib)).unwrap();
    let lib_map = lib.as_table().unwrap();
    let target_dir = lib_map.get(stringify!(jvm_target_dir)).unwrap();
    let target_dir_path = target_dir.as_str().unwrap();

    println!("cargo:rustc-env=process_dir={}", process_dir_path.display());
    println!("cargo:rustc-env=out_dir={}", target_dir_path);
    println!("cargo:rustc-cfg=bytecode_build");
    println!("cargo:rustc-link-search={path}lib");
    println!("cargo:rustc-link-lib=jvm");
}