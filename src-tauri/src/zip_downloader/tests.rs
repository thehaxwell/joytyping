use std::{path::PathBuf, io::{self, ErrorKind}};

use bytes::Bytes;
use mockall::predicate::eq;
use zip::result::ZipError;

use crate::{file_wrapper::MockFileWrapperTrait, zip_downloader::{ZipDownloaderError, ZipDownloaderStage, zip_archive_wrapper::ZipArchiveError, reqwest_wrapper}};

use super::{zip_archive_wrapper::MockZipArchiveWrapperTrait, reqwest_wrapper::MockReqwestWrapperTrait, ZipDownloader, ZipDownloaderTrait};

#[test]
fn download_and_unzip_works() {
    let mut mock_zip_archive = MockZipArchiveWrapperTrait::new();
    let mut mock_reqwest_wrapper = MockReqwestWrapperTrait::new();
    let mut mock_file_handler = MockFileWrapperTrait::new();

    let zip_url = "www.joytyping.com/download-zip".to_string();
	let temp_zip_file_name = "joytyping-zip-download".to_string();
	let zip_save_path = PathBuf::from("/home/haxwell/downloads");
	let zip_file_path = PathBuf::from("/home/haxwell/downloads/joytyping-zip-download.zip");

    mock_reqwest_wrapper
        .expect_get_as_bytes()
        .with(eq(zip_url.clone()))
        .times(1)
        .returning(|_| Ok(Bytes::from("fj wae90ff aksdjfeifjn")));

    mock_file_handler
        .expect_create_and_write_all()
        .with(eq(zip_file_path.clone()),eq(Bytes::from("fj wae90ff aksdjfeifjn")))
        .times(1)
        .returning(|_,_| Ok(()));

    mock_zip_archive
        .expect_unzip()
        .with(eq(zip_file_path.clone()))
        .times(1)
        .returning(|_| Ok(()));

    mock_file_handler
        .expect_remove_path()
        .with(eq(zip_file_path.clone()))
        .times(1)
        .returning(|_| Ok(()));

    let mut zip_downloader = ZipDownloader::new(
        Box::new(mock_zip_archive),
        Box::new(mock_reqwest_wrapper),
        Box::new(mock_file_handler),);

    assert_eq!(
        zip_downloader
           .download_and_unzip(
               zip_url,
               temp_zip_file_name,
               zip_save_path).unwrap(),());
}

#[test]
fn download_and_unzip_handles_remove_path_errors() {
    let mut mock_zip_archive = MockZipArchiveWrapperTrait::new();
    let mut mock_reqwest_wrapper = MockReqwestWrapperTrait::new();
    let mut mock_file_handler = MockFileWrapperTrait::new();

    let zip_url = "www.joytyping.com/download-zip".to_string();
	let temp_zip_file_name = "joytyping-zip-download".to_string();
	let zip_save_path = PathBuf::from("/home/haxwell/downloads");
	let zip_file_path = PathBuf::from("/home/haxwell/downloads/joytyping-zip-download.zip");

    mock_reqwest_wrapper
        .expect_get_as_bytes()
        .with(eq(zip_url.clone()))
        .times(1)
        .returning(|_| Ok(Bytes::from("fj wae90ff aksdjfeifjn")));

    mock_file_handler
        .expect_create_and_write_all()
        .with(eq(zip_file_path.clone()),eq(Bytes::from("fj wae90ff aksdjfeifjn")))
        .times(1)
        .returning(|_,_| Ok(()));

    mock_zip_archive
        .expect_unzip()
        .with(eq(zip_file_path.clone()))
        .times(1)
        .returning(|_| Ok(()));

    mock_file_handler
        .expect_remove_path()
        .with(eq(zip_file_path.clone()))
        .times(1)
        .returning(|_| Err(io::Error::new(
                    ErrorKind::AddrNotAvailable,"eish! remove_path failed")));

    let mut zip_downloader = ZipDownloader::new(
        Box::new(mock_zip_archive),
        Box::new(mock_reqwest_wrapper),
        Box::new(mock_file_handler),);

    assert_eq!(
        zip_downloader
           .download_and_unzip(
               zip_url,
               temp_zip_file_name,
               zip_save_path).unwrap_err(),
           ZipDownloaderError::IO(io::Error::new(
                    ErrorKind::AddrNotAvailable,"eish! remove_path failed")
               ,ZipDownloaderStage::DeleteSource));
}

#[test]
fn download_and_unzip_handles_unzip_io_errors() {
    let mut mock_zip_archive = MockZipArchiveWrapperTrait::new();
    let mut mock_reqwest_wrapper = MockReqwestWrapperTrait::new();
    let mut mock_file_handler = MockFileWrapperTrait::new();

    let zip_url = "www.joytyping.com/download-zip".to_string();
	let temp_zip_file_name = "joytyping-zip-download".to_string();
	let zip_save_path = PathBuf::from("/home/haxwell/downloads");
	let zip_file_path = PathBuf::from("/home/haxwell/downloads/joytyping-zip-download.zip");

    mock_reqwest_wrapper
        .expect_get_as_bytes()
        .with(eq(zip_url.clone()))
        .times(1)
        .returning(|_| Ok(Bytes::from("fj wae90ff aksdjfeifjn")));

    mock_file_handler
        .expect_create_and_write_all()
        .with(eq(zip_file_path.clone()),eq(Bytes::from("fj wae90ff aksdjfeifjn")))
        .times(1)
        .returning(|_,_| Ok(()));

    mock_zip_archive
        .expect_unzip()
        .with(eq(zip_file_path.clone()))
        .times(1)
        .returning(|_| Err(ZipArchiveError::IO(io::Error::new(
                    ErrorKind::AddrNotAvailable,"eish! remove_path failed"))));

    let mut zip_downloader = ZipDownloader::new(
        Box::new(mock_zip_archive),
        Box::new(mock_reqwest_wrapper),
        Box::new(mock_file_handler),);

    assert_eq!(
        zip_downloader
           .download_and_unzip(
               zip_url,
               temp_zip_file_name,
               zip_save_path).unwrap_err(),
           ZipDownloaderError::IO(io::Error::new(
                    ErrorKind::AddrNotAvailable,"eish! remove_path failed")
               ,ZipDownloaderStage::Unzip));
}

#[test]
fn download_and_unzip_handles_unzip_zip_errors() {
    let mut mock_zip_archive = MockZipArchiveWrapperTrait::new();
    let mut mock_reqwest_wrapper = MockReqwestWrapperTrait::new();
    let mut mock_file_handler = MockFileWrapperTrait::new();

    let zip_url = "www.joytyping.com/download-zip".to_string();
	let temp_zip_file_name = "joytyping-zip-download".to_string();
	let zip_save_path = PathBuf::from("/home/haxwell/downloads");
	let zip_file_path = PathBuf::from("/home/haxwell/downloads/joytyping-zip-download.zip");

    mock_reqwest_wrapper
        .expect_get_as_bytes()
        .with(eq(zip_url.clone()))
        .times(1)
        .returning(|_| Ok(Bytes::from("fj wae90ff aksdjfeifjn")));

    mock_file_handler
        .expect_create_and_write_all()
        .with(eq(zip_file_path.clone()),eq(Bytes::from("fj wae90ff aksdjfeifjn")))
        .times(1)
        .returning(|_,_| Ok(()));

    mock_zip_archive
        .expect_unzip()
        .with(eq(zip_file_path.clone()))
        .times(1)
        .returning(|_| Err(ZipArchiveError::Zip(ZipError::Io(io::Error::new(
                    ErrorKind::AddrNotAvailable,"eish! remove_path failed")))));

    let mut zip_downloader = ZipDownloader::new(
        Box::new(mock_zip_archive),
        Box::new(mock_reqwest_wrapper),
        Box::new(mock_file_handler),);

    assert_eq!(
        zip_downloader
           .download_and_unzip(
               zip_url,
               temp_zip_file_name,
               zip_save_path).unwrap_err(),
           ZipDownloaderError::Zip(ZipError::Io(io::Error::new(
                    ErrorKind::AddrNotAvailable,"eish! remove_path failed"))
               ,ZipDownloaderStage::Unzip));
}

#[test]
fn download_and_unzip_handles_create_and_write_all_errors() {
    let mock_zip_archive = MockZipArchiveWrapperTrait::new();
    let mut mock_reqwest_wrapper = MockReqwestWrapperTrait::new();
    let mut mock_file_handler = MockFileWrapperTrait::new();

    let zip_url = "www.joytyping.com/download-zip".to_string();
	let temp_zip_file_name = "joytyping-zip-download".to_string();
	let zip_save_path = PathBuf::from("/home/haxwell/downloads");
	let zip_file_path = PathBuf::from("/home/haxwell/downloads/joytyping-zip-download.zip");

    mock_reqwest_wrapper
        .expect_get_as_bytes()
        .with(eq(zip_url.clone()))
        .times(1)
        .returning(|_| Ok(Bytes::from("fj wae90ff aksdjfeifjn")));

    mock_file_handler
        .expect_create_and_write_all()
        .with(eq(zip_file_path.clone()),eq(Bytes::from("fj wae90ff aksdjfeifjn")))
        .times(1)
        .returning(|_,_| Err(io::Error::new(
            ErrorKind::AddrNotAvailable,"eish! remove_path failed")) );

    let mut zip_downloader = ZipDownloader::new(
        Box::new(mock_zip_archive),
        Box::new(mock_reqwest_wrapper),
        Box::new(mock_file_handler),);

    assert_eq!(
        zip_downloader
           .download_and_unzip(
               zip_url,
               temp_zip_file_name,
               zip_save_path).unwrap_err(),
           ZipDownloaderError::IO(io::Error::new(
                    ErrorKind::AddrNotAvailable,"eish! remove_path failed")
               ,ZipDownloaderStage::DownloadZip));
}

#[test]
fn download_and_unzip_handles_get_as_bytes_errors() {
    let mock_zip_archive = MockZipArchiveWrapperTrait::new();
    let mut mock_reqwest_wrapper = MockReqwestWrapperTrait::new();
    let mock_file_handler = MockFileWrapperTrait::new();

    let zip_url = "https://www.joytyping.com/download-zip".to_string();
	let temp_zip_file_name = "joytyping-zip-download".to_string();
	let zip_save_path = PathBuf::from("/home/haxwell/downloads");

    mock_reqwest_wrapper
        .expect_get_as_bytes()
        .with(eq(zip_url.clone()))
        .times(1)
        .returning(|_| Err(reqwest_wrapper::Error{
            kind: reqwest_wrapper::ErrorKind::Connect,
            url: Some(reqwest::Url::parse("https://www.joytyping.com").unwrap())
        }));

    let mut zip_downloader = ZipDownloader::new(
        Box::new(mock_zip_archive),
        Box::new(mock_reqwest_wrapper),
        Box::new(mock_file_handler),);

    assert_eq!(
        zip_downloader
           .download_and_unzip(
               zip_url,
               temp_zip_file_name,
               zip_save_path).unwrap_err(),
           ZipDownloaderError::Http(
               reqwest_wrapper::Error{
                    kind: reqwest_wrapper::ErrorKind::Connect,
                    url: Some(reqwest::Url::parse("https://www.joytyping.com").unwrap())
                }
               ,ZipDownloaderStage::DownloadZip));
}
