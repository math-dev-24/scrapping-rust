use serde::Serialize;


#[derive(Serialize, Debug)]
pub struct ProductData {
    pub code: String,
    pub features: Vec<Features>
}

#[derive(Serialize, Debug)]
pub struct Features {
    pub key: String,
    pub value: String
}
