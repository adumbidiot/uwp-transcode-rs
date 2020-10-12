use crate::{
    TranscodeError,
    TranscodeResult,
};
use std::{
    ffi::{
        OsStr,
        OsString,
    },
    iter::once,
    os::windows::ffi::{
        OsStrExt,
        OsStringExt,
    },
    path::Path,
};
use winapi::{
    shared::minwindef::MAX_PATH,
    um::fileapi::GetFullPathNameW,
};
pub use windows_media_transcoding_bindings::windows::storage::{
    CreationCollisionOption,
    StorageFile,
    StorageFolder,
};

/// Normalize a utf8 path. Fails if normalized path is not utf8.
pub fn normalize_path(path: &str) -> TranscodeResult<String> {
    let path = OsStr::new(path)
        .encode_wide()
        .chain(once(0))
        .collect::<Vec<u16>>();

    let path = unsafe {
        let mut ret = [0; MAX_PATH + 1];
        GetFullPathNameW(
            path.as_ptr(),
            ret.len() as u32,
            ret.as_mut_ptr(),
            std::ptr::null_mut(),
        );

        let end = ret.iter().position(|el| *el == 0).unwrap_or(MAX_PATH);

        OsString::from_wide(&ret[..end])
    };

    let path = path.to_str().ok_or(TranscodeError::NonUtf8Path)?;

    Ok(path.to_string())
}

/// Options for when a created file has the same name as another.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum CreationOptions {
    /// Creates a new name
    CreateUniqueName,

    /// Overwrites
    Overwrite,

    /// Fails operation
    Fail,

    /// Opens old file
    Open,
}

impl Default for CreationOptions {
    fn default() -> Self {
        Self::Overwrite
    }
}

impl Into<CreationCollisionOption> for CreationOptions {
    fn into(self) -> CreationCollisionOption {
        match self {
            Self::CreateUniqueName => CreationCollisionOption::GenerateUniqueName,
            Self::Overwrite => CreationCollisionOption::ReplaceExisting,
            Self::Fail => CreationCollisionOption::FailIfExists,
            Self::Open => CreationCollisionOption::OpenIfExists,
        }
    }
}

/// A File wrapper
#[derive(Debug, Clone)]
pub struct File {
    pub(crate) file: StorageFile,
}

impl File {
    /// Open a file at the location.
    pub async fn open(path: &str) -> TranscodeResult<Self> {
        let path = normalize_path(path)?;
        let file = StorageFile::get_file_from_path_async(path)?.await?;

        Ok(File { file })
    }

    /// Create a file at the location.
    pub async fn create(path: &str, options: CreationOptions) -> TranscodeResult<Self> {
        let path = normalize_path(path)?;
        let path = Path::new(&path);

        let folder_path = path
            .parent()
            .expect("Directory Parent")
            .to_str()
            .ok_or(TranscodeError::NonUtf8Path)?;

        let folder = StorageFolder::get_folder_from_path_async(folder_path)?.await?;

        let file_name = Path::new(path)
            .file_name()
            .expect("File Name")
            .to_str()
            .ok_or(TranscodeError::NonUtf8Path)?;

        let file = folder.create_file_async(file_name, options.into())?.await?;

        Ok(Self { file })
    }

    /// Get the inner [`StorageFile`] object.
    pub fn as_storage_file(&self) -> &StorageFile {
        &self.file
    }
}
