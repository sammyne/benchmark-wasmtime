#[cfg(test)]
mod tests {
    use crate::{json_to_wasm_value, load_component, parse_json_params, wasm_value_to_json};
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;
    use wasmtime_v41::component::Val;

    #[test]
    fn test_parse_json_params_valid() {
        let params = vec![
            "42".to_string(),
            "\"hello\"".to_string(),
            "true".to_string(),
        ];
        let result = parse_json_params(&params);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed[0], 42);
        assert_eq!(parsed[1], "hello");
        assert_eq!(parsed[2], true);
    }

    #[test]
    fn test_parse_json_params_invalid() {
        let params = vec!["{invalid json}".to_string()];
        let result = parse_json_params(&params);
        assert!(result.is_err());
    }

    #[test]
    fn test_json_to_wasm_value_bool() {
        let json_val = serde_json::Value::Bool(true);
        let result = json_to_wasm_value(&json_val, 0);
        assert!(result.is_ok());
        matches!(result.unwrap(), Val::Bool(true));
    }

    #[test]
    fn test_json_to_wasm_value_number_i32() {
        let json_val = serde_json::Value::Number(42.into());
        let result = json_to_wasm_value(&json_val, 0);
        assert!(result.is_ok());
        matches!(result.unwrap(), Val::S32(42));
    }

    #[test]
    fn test_json_to_wasm_value_number_f64() {
        let json_val = serde_json::Value::Number(serde_json::Number::from_f64(3.14).unwrap());
        let result = json_to_wasm_value(&json_val, 0);
        assert!(result.is_ok());
        if let Val::Float64(f) = result.unwrap() {
            assert!((f - 3.14).abs() < 0.01);
        } else {
            panic!("Expected Float64");
        }
    }

    #[test]
    fn test_json_to_wasm_value_string() {
        let json_val = serde_json::Value::String("test".to_string());
        let result = json_to_wasm_value(&json_val, 0);
        assert!(result.is_ok());
        if let Val::String(s) = result.unwrap() {
            assert_eq!(s, "test");
        } else {
            panic!("Expected String");
        }
    }

    #[test]
    fn test_json_to_wasm_value_array_unsupported() {
        let json_val = serde_json::Value::Array(vec![]);
        let result = json_to_wasm_value(&json_val, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_json_to_wasm_value_object_unsupported() {
        let json_val = serde_json::Value::Object(serde_json::Map::new());
        let result = json_to_wasm_value(&json_val, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_wasm_value_to_json_bool() {
        let wasm_val = Val::Bool(true);
        let result = wasm_value_to_json(&wasm_val);
        assert_eq!(result, serde_json::Value::Bool(true));
    }

    #[test]
    fn test_wasm_value_to_json_s32() {
        let wasm_val = Val::S32(42);
        let result = wasm_value_to_json(&wasm_val);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_wasm_value_to_json_string() {
        let wasm_val = Val::String("test".to_string());
        let result = wasm_value_to_json(&wasm_val);
        assert_eq!(result, "test");
    }

    #[test]
    fn test_load_component_file_not_found() {
        let path = PathBuf::from("/nonexistent/file.wasm");
        let result = load_component(&path);
        match result {
            Ok(_) => panic!("Expected error"),
            Err(e) => assert!(e.to_string().contains("not found")),
        }
    }

    #[test]
    fn test_load_component_invalid_wasm() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "invalid wasm content").unwrap();

        let result = load_component(&temp_file.path().to_path_buf());
        match result {
            Ok(_) => panic!("Expected error"),
            Err(e) => assert!(e.to_string().contains("Failed to load WASM component")),
        }
    }
}
