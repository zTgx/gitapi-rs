extern crate gitapi_rs as Api;
use Api::core::engine::Engine;
use Api::http::path::Path;

fn main() {
    let mut engine = Engine::new();
    let path = Path::new().slash(&"users".to_owned()).slash(&"user".to_owned()).slash(&"repos".to_owned()).ok();
    let profile = engine.get(&path).unwrap();
    println!("profile: {:?}", profile);
}
