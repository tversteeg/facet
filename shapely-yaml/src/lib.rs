#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use shapely_core::Partial;
use yaml_rust2::YamlLoader;

/// Deserializes YAML data into a Shapely Partial.
///
/// # Example
///
/// ```
/// use shapely::Shapely;
/// use shapely_yaml::from_yaml;
///
/// #[derive(Debug, Shapely, PartialEq)]
/// struct Config {
///     name: String,
///     version: u64,
/// }
///
/// let yaml = r#"
/// name: MyApp
/// version: 1
/// "#;
///
/// let mut partial = Config::partial();
/// from_yaml(&mut partial, yaml).expect("Failed to parse YAML");
///
/// let config = partial.build::<Config>();
/// assert_eq!(config, Config { name: "MyApp".to_string(), version: 1 });
/// ```
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
