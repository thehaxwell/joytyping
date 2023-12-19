use std::{fs::{File, self}, path::{Path, PathBuf}, io};

use zip::result::ZipError;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait ZipArchiveWrapperTrait {
    fn unzip(&self, zip_file_path: &Path) -> Result<(),ZipArchiveError>;
}

//TODO: make this a proper thin wrapper around zip::ZipArchive
pub struct ZipArchiveWrapper;

impl ZipArchiveWrapperTrait for ZipArchiveWrapper {
    fn unzip(&self, zip_file_path: &Path) -> Result<(),ZipArchiveError> {
        let file = File::open(zip_file_path)
            .map_err(|e| ZipArchiveError::IO(e))?;
        let mut archive = zip::ZipArchive::new(file)
            .map_err(|e| ZipArchiveError::Zip(e))?;

        let outpath_base = PathBuf::from(
            zip_file_path.parent().unwrap_or(Path::new("/")));
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .map_err(|e| ZipArchiveError::Zip(e))?;

            let outpath = match file.enclosed_name() {
                Some(path) => {
                    let mut outpath = outpath_base.clone();
                    outpath.push(path);
                    outpath
                },
                None => continue,
            };
            //
            // {
            //     let comment = file.comment();
            //     if !comment.is_empty() {
            //         println!("File {i} comment: {comment}");
            //     }
            // }

            if (*file.name()).ends_with('/') {
                // println!("File {} extracted to \"{}\"", i, outpath.display());
                fs::create_dir_all(&outpath)
                    .map_err(|e| ZipArchiveError::IO(e))?;
            } else {
                // println!(
                //     "File {} extracted to \"{}\" ({} bytes)",
                //     i,
                //     outpath.display(),
                //     file.size()
                // );
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p)
                            .map_err(|e| ZipArchiveError::IO(e))?;
                    }
                }
                let mut outfile = fs::File::create(&outpath)
                    .map_err(|e| ZipArchiveError::IO(e))?;
                io::copy(&mut file, &mut outfile)
                    .map_err(|e| ZipArchiveError::IO(e))?;
            }

            // Get and Set permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))
                        .map_err(|e| ZipArchiveError::IO(e))?;
                }
            }
        }
        Ok(())
    }
}

pub enum ZipArchiveError {
    IO(io::Error),
    Zip(ZipError)
}

