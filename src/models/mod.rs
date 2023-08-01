use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct PictureFile {
    pub pid: String,
    pub url: String,
    pub owner: String,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct PictureDiscription {
    pub pid: String,
    pub discription: String,
}