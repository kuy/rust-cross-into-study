use domain::Cat;
use gateway::CatJson;

fn main() {
    let json = CatJson { name: "Tama".into() };
    println!("JSON: {:?}", json);

    let cat: Cat = json.into();
    println!("Cat: {:?}", cat);
}
