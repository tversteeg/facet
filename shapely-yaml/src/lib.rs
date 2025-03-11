#![doc = include_str!("../README.md")]

use shapely_core::Partial;
use yaml_rust2::YamlLoader;

pub fn from_yaml(partial: &mut Partial, yaml: &str) -> Result<(), String> {
    let docs = YamlLoader::load_from_str(yaml).map_err(|e| e.to_string())?;

    // Assuming only one document in the YAML string
    let doc = &docs[0];
    match doc {
        yaml_rust2::Yaml::Hash(hash) => {
            for (key, value) in hash {
                let key_str = match key.as_str() {
                    Some(s) => s,
                    None => return Err("Expected string key".to_string()),
                };

                let slot = partial
                    .slot_by_name(key_str)
                    .map_err(|_| format!("Unknown field: {}", key_str))?;

                match value {
                    yaml_rust2::Yaml::String(s) => {
                        slot.fill(s.clone());
                    }
                    yaml_rust2::Yaml::Integer(i) => {
                        slot.fill(*i as u64);
                    }
                    _ => {
                        println!("Unsupported YAML type for field: {}", key_str);
                        unimplemented!()
                    }
                }
            }
        }
        _ => {
            return Err("Expected a YAML hash".to_string());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
