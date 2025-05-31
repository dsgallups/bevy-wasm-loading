use walkdir::WalkDir;

fn main() {
    let mut assets = Vec::new();

    for entry in WalkDir::new("assets") {
        let Ok(entry) = entry else {
            continue;
        };
        if entry.path().extension().is_some_and(|ext| {
            ext.to_str()
                .is_some_and(|ext| ext == "gltf" || ext == "glb")
        }) {
            assets.push(entry.into_path());
        }
    }
    print!("[");
    for path in assets {
        let Some(str_path) = path.to_str() else {
            continue;
        };
        let Some((_, str_path)) = str_path.split_once("/") else {
            continue;
        };
        print!(r#""{}","#, str_path);
    }
    println!("]");
}
