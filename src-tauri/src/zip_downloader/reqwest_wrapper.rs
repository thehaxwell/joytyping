use bytes:: Bytes;
use reqwest::Error;

pub trait ReqwestWrapperTrait {
    fn get_as_bytes(&self,path: String) -> Result<Bytes,Error>;
}

pub struct ReqwestWrapper;

impl ReqwestWrapperTrait for ReqwestWrapper {
    fn get_as_bytes(&self,path: String) -> Result<Bytes,Error>{
        let resp = reqwest::blocking::get(path)?;
        resp.bytes()
    }
}
