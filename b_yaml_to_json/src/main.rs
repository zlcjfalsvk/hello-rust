use std::fs;

fn yaml_to_json(yaml_path: &str, json_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let yaml_content = fs::read_to_string(yaml_path)?;
    let yaml_data: serde_yaml::Value = serde_yaml::from_str(&yaml_content)?;
    let json_data = serde_json::to_string_pretty(&yaml_data)?;
    fs::write(json_path, json_data)?;
    Ok(())
}

fn json_to_yaml(json_path: &str, yaml_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json_content = fs::read_to_string(json_path)?;
    let json_data: serde_json::Value = serde_json::from_str(&json_content)?;
    let yaml_data = serde_yaml::to_string(&json_data)?;
    fs::write(yaml_path, yaml_data)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    yaml_to_json(
        "b_yaml_to_json/src/data/data.yaml",
        "b_yaml_to_json/src/data/data_converted.json",
    )?;
    json_to_yaml(
        "b_yaml_to_json/src/data/data.json",
        "b_yaml_to_json/src/data/data_converted.yaml",
    )?;
    Ok(())
}
