use std::io;
use std::path::PathBuf;

use crate::file_wrapper::FileWrapperTrait;

use self::reqwest_wrapper::ReqwestWrapperTrait;
use self::zip_archive_wrapper::{ZipArchiveWrapperTrait, ZipArchiveError};

use thiserror::Error;

pub mod zip_archive_wrapper;
pub mod reqwest_wrapper;

pub trait ZipDownloaderTrait {
    fn download_and_unzip(&mut self,
		zip_url: String,
		temp_zip_file_name: String,
		zip_save_path: PathBuf,
    ) -> Result<(),ZipDownloaderError>;
}

pub struct ZipDownloader {
    zip_archive: Box<dyn ZipArchiveWrapperTrait>,
    http_client: Box<dyn ReqwestWrapperTrait>,
    file_handler: Box<dyn FileWrapperTrait>,
}

impl ZipDownloader {
    pub fn new(
        zip_archive: Box<dyn ZipArchiveWrapperTrait>,
        http_client: Box<dyn ReqwestWrapperTrait>,
        file_handler: Box<dyn FileWrapperTrait>,
    ) -> Self {
        Self {
            zip_archive,
            http_client,
            file_handler,
        }
    }

}

impl ZipDownloaderTrait for ZipDownloader {
    fn download_and_unzip(&mut self,
		zip_url: String,
		temp_zip_file_name: String,
		zip_save_path: PathBuf,
    ) -> Result<(),ZipDownloaderError>{
        // unzip the zip_archive
        let body = self.http_client.get_as_bytes(zip_url)
            .map_err(|e|ZipDownloaderError::Http(e,ZipDownloaderStage::DownloadZip))?;

        let mut zip_file_path = zip_save_path;
            zip_file_path.push(temp_zip_file_name);
            zip_file_path.set_extension("zip");
        self.file_handler.create_and_write_all(&zip_file_path,&body)
            .map_err(|e|ZipDownloaderError::IO(e,ZipDownloaderStage::DownloadZip))?;

        // unzip the archive
        self.zip_archive.unzip(&zip_file_path)
            .map_err(|err|match err {
                ZipArchiveError::IO(e) 
                    => ZipDownloaderError::IO(e,ZipDownloaderStage::Unzip),
                ZipArchiveError::Zip(e) 
                    => ZipDownloaderError::Zip(e,ZipDownloaderStage::Unzip),
            })?;

        // delete the source file
        self.file_handler.remove_path(&zip_file_path)
            .map_err(|e|ZipDownloaderError::IO(e,ZipDownloaderStage::DeleteSource))?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum ZipDownloaderStage {
    DownloadZip,
    Unzip,
    DeleteSource
}

#[derive(Error, Debug)]
pub enum ZipDownloaderError {
    #[error("IO error while {1:?}: {0}")]
    IO(io::Error,ZipDownloaderStage),

    #[error("Zip error while {1:?}: {0}")]
    Zip(zip::result::ZipError,ZipDownloaderStage),

    #[error("Http error while {1:?}: {0}")]
    Http(reqwest::Error,ZipDownloaderStage),
}

