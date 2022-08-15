use domain::Cat;

#[derive(Debug)]
pub struct CatJson {
    pub name: String
}
// Works!
// impl Into<Cat> for CatJson {
//     fn into(self) -> Cat {
//         Cat { name: self.name }
//     }
// }

// Works!
impl From<CatJson> for Cat {
    fn from(json: CatJson) -> Self {
        Self { name: json.name }
    }
}
