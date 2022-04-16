use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[macro_use]
extern crate rocket;
use rocket::{data::ToByteUnit, Data};

pub enum Item {
    Folder(Folder),
    File(Vec<u8>),
}

pub struct Folder(HashMap<String, Item>);

pub struct Project {
    root: Folder,
}

pub struct DbProjects {
    projects: sled::Db,
}

#[get("/")]
fn home() -> &'static str {
    "Welcome home!"
}

#[get("/<name>/preview/<path..>")]
fn preview(name: &str, path: PathBuf) -> &'static str {
    "Hello, world!"
}

#[post("/<name>/upload", data = "<project>")]
fn upload(name: &str, project: Data<'_>) -> &'static str {
    todo!()
}

pub struct Addr([u8; 32]);

impl Addr {
    pub fn hash(bytes: &[u8]) -> Addr {
        let hash = blake3::hash(bytes).as_bytes().to_owned();
        Addr(hash)
    }
}

/// A content-addressed database
pub struct DbContent {
    blobs: sled::Db,
}

impl DbContent {
    pub fn open<P: AsRef<Path>>(path: P) -> Option<DbContent> {
        let blobs = sled::open(path).ok()?;
        Some(DbContent { blobs })
    }

    pub fn insert(&mut self, blob: &[u8]) -> Addr {
        let addr = Addr::hash(blob);
        let _old = self.blobs.insert(addr.0, blob).unwrap();
        return addr;
    }

    pub fn get(&self, addr: Addr) -> Option<sled::IVec> {
        self.blobs.get(addr.0).unwrap()
    }
}

#[launch]
fn rocket() -> _ {
    let mut content = DbContent::open("db_content").unwrap();
    content.insert(b"hello kind old man...");

    rocket::build().mount("/", routes![preview, home, upload])
}
