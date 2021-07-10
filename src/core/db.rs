use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sled::Db as DBHandle;
use std::fs;
use std::io;
use std::marker::PhantomData;
use std::path::PathBuf;
use uuid::Uuid;

pub struct Database {
    filepath: PathBuf,
    handle: DBHandle,
}

impl Database {
    pub fn open_or_create<P: Into<PathBuf>>(path: P) -> Result<Self, io::Error> {
        let filepath = path.into();
        let handle = sled::open(&filepath).map_err(|err| {
            match err {
                sled::Error::Io(err) => err,
                _ => unreachable!("A non I/O related error was found while creating the database. Such error should not be possible. Please report the incident.")
            }
        })?;
        Ok(Self { filepath, handle })
    }

    pub fn delete(self) -> io::Result<()> {
        fs::remove_dir_all(self.filepath)
    }
}

pub struct DBId<T> {
    id: Uuid,
    marker: PhantomData<T>,
}

impl<T> Serialize for DBId<T> {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        self.id.serialize(ser)
    }
}

impl<'de, T> Deserialize<'de> for DBId<T> {
    fn deserialize<D: Deserializer<'de>>(de: D) -> std::result::Result<Self, D::Error> {
        Ok(Self {
            id: Uuid::deserialize(de)?,
            marker: PhantomData,
        })
    }
}
