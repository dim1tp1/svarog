use std::fs;
use std::path::Path;

use glob::glob;
use tonic_build;
use xuanmi_base_support::*;

fn main() -> Outcome<()> {
    '_validate_running_in_the_right_place: {
        assert_throw!(Path::new("proto").is_dir());
        assert_throw!(Path::new("src").is_dir());
        assert_throw!(Path::new("Cargo.toml").exists());
    }

    '_clean: {
        let path = Path::new("src/protogen");
        if path.is_dir() {
            fs::remove_dir_all(path).catch_()?;
        }
        fs::create_dir("src/protogen").catch_()?;
    }

    '_proto_to_rs: {
        let mut protos: Vec<String> = Vec::new();
        for entry in glob("proto/*.proto").catch_()? {
            if let Ok(path) = entry {
                protos.push(path.to_str().unwrap().to_string());
            }
        }
        tonic_build::configure()
            .out_dir("src/protogen")
            .compile(&protos, &["proto"])
            .catch_()?;
    }

    '_protogen_mod_rs: {
        let mut mod_rs: Vec<String> = Vec::new();
        for entry in glob("src/protogen/*.rs").catch_()? {
            if let Ok(path) = entry {
                let filename = path.file_stem().unwrap().to_str().unwrap();
                mod_rs.push(format!("pub mod {};", filename));
            }
        }
        // Concat mod_rs into a single string.
        let mod_rs = mod_rs.join("\n");
        println!("{}", &mod_rs);
        // Write mod_rs to src/protogen/mod.rs.
        fs::write("src/protogen/mod.rs", mod_rs).catch_()?;
    }

    Ok(())
}
