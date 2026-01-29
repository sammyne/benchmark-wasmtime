use anyhow::{Context, Result};
use clap::Parser;
use std::convert::From;
use std::path::PathBuf;
use wasmtime_v41::{Config, Engine, Store, component::*};
use wasmtime_wasi_v41::p2::add_to_linker_sync;
use wasmtime_wasi_v41::{WasiCtx, WasiCtxView, WasiView};

mod tests;

/// Simple WasiView implementation for WasiCtx
#[derive(Default)]
struct MyState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for MyState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.ctx,
            table: &mut self.table,
        }
    }
}

/// CLI tool for running WebAssembly Component functions
#[derive(Parser, Debug)]
#[command(name = "wasmtime-cli")]
#[command(about = "Run WebAssembly Component functions with JSON parameters", long_about = None)]
struct Args {
    /// Path to the WASM Component file
    #[arg(short, long, value_name = "FILE")]
    wasm: PathBuf,

    /// Name of the function to execute
    #[arg(short, long, value_name = "FUNCTION")]
    function: String,

    /// JSON parameters to pass to the function (positional)
    #[arg(value_name = "JSON")]
    params: Vec<String>,
}

/// Load and validate a WASM Component file
///
/// The file must be a pre-packaged WASM component (binary format).
///
/// # Arguments
/// * `path` - Path to the WASM Component file
///
/// # Returns
/// The loaded component and engine
fn load_component(path: &PathBuf) -> Result<(Component, Engine)> {
    // Check if file exists
    if !path.exists() {
        anyhow::bail!("WASM file not found: {}", path.display());
    }

    // Check if file is readable
    if !path.is_file() {
        anyhow::bail!("Path is not a file: {}", path.display());
    }

    // Create wasmtime engine with component model support
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.wasm_simd(true);
    let engine = Engine::new(&config).context("Failed to create wasmtime engine")?;

    // Load the component directly from file
    let component = Component::from_file(&engine, path)
        .with_context(|| format!("Failed to load WASM component from: {}", path.display()))?;

    Ok((component, engine))
}

/// Parse JSON parameters from command line arguments
///
/// # Arguments
/// * `params` - Vector of JSON string parameters
///
/// # Returns
/// Vector of parsed JSON values
pub fn parse_json_params(params: &[String]) -> Result<Vec<serde_json::Value>> {
    let mut parsed_params = Vec::with_capacity(params.len());

    for (i, param) in params.iter().enumerate() {
        let value: serde_json::Value = serde_json::from_str(param)
            .with_context(|| format!("Failed to parse JSON at position {}: '{}'", i + 1, param))?;
        parsed_params.push(value);
    }

    Ok(parsed_params)
}

/// Convert JSON value to WASM value based on expected type
///
/// # Arguments
/// * `value` - JSON value to convert
/// * `expect_type` - Expected WASM type for the conversion
///
/// # Returns
/// The converted value as a Val
pub fn json_to_wasm_value(value: serde_json::Value, expect_type: &Type) -> Result<Val> {
    use serde_json::Value;

    match (expect_type, value) {
        (Type::Bool, Value::Bool(v)) => Ok(Val::Bool(v)),
        (Type::S8, Value::Number(n)) => n
            .as_i64()
            .and_then(|i| i.try_into().ok())
            .map(Val::S8)
            .ok_or_else(|| anyhow::anyhow!("Expected s8 integer")),
        (Type::S16, Value::Number(n)) => n
            .as_i64()
            .and_then(|i| i.try_into().ok())
            .map(Val::S16)
            .ok_or_else(|| anyhow::anyhow!("Expected s16 integer")),
        (Type::S32, Value::Number(n)) => n
            .as_i64()
            .and_then(|i| i.try_into().ok())
            .map(Val::S32)
            .ok_or_else(|| anyhow::anyhow!("Expected s32 integer")),
        (Type::S64, Value::Number(n)) => n
            .as_i64()
            .map(Val::S64)
            .ok_or_else(|| anyhow::anyhow!("Expected s64 integer")),
        (Type::U8, Value::Number(n)) => n
            .as_u64()
            .and_then(|u| u.try_into().ok())
            .map(Val::U8)
            .ok_or_else(|| anyhow::anyhow!("Expected u8 integer")),
        (Type::U16, Value::Number(n)) => n
            .as_u64()
            .and_then(|u| u.try_into().ok())
            .map(Val::U16)
            .ok_or_else(|| anyhow::anyhow!("Expected u16 integer")),
        (Type::U32, Value::Number(n)) => n
            .as_u64()
            .and_then(|u| u.try_into().ok())
            .map(Val::U32)
            .ok_or_else(|| anyhow::anyhow!("Expected u32 integer")),
        (Type::U64, Value::Number(n)) => n
            .as_u64()
            .map(Val::U64)
            .ok_or_else(|| anyhow::anyhow!("Expected u64 integer")),
        (Type::Float32, Value::Number(n)) => n
            .as_f64()
            .map(|f| Val::Float32(f as f32))
            .ok_or_else(|| anyhow::anyhow!("Expected number")),
        (Type::Float64, Value::Number(n)) => n
            .as_f64()
            .map(Val::Float64)
            .ok_or_else(|| anyhow::anyhow!("Expected number")),
        (Type::Char, Value::String(s)) => {
            if s.len() == 1 {
                Ok(Val::Char(s.chars().next().unwrap()))
            } else {
                anyhow::bail!("Expected single character string")
            }
        }
        (Type::String, Value::String(s)) => Ok(Val::String(s)),
        (Type::List(v), Value::Array(arr)) => {
            let elem_type = v.ty();

            // Convert JSON Array to WASM List
            let mut wasm_values = Vec::new();
            for elem in arr {
                wasm_values.push(json_to_wasm_value(elem, &elem_type)?);
            }

            Ok(Val::List(wasm_values))
        }
        (Type::Record(r), Value::Object(mut obj)) => {
            let mut wasm_values = Vec::new();

            for f in r.fields() {
                let v = obj
                    .remove(f.name)
                    .ok_or_else(|| anyhow::anyhow!("Field {} not found", f.name))?;

                let w = json_to_wasm_value(v, &f.ty)
                    .with_context(|| format!("bad field {}", f.name))?;

                wasm_values.push((f.name.to_owned(), w));
            }

            Ok(Val::Record(wasm_values))
        }
        (expect, got) => anyhow::bail!("expect {expect:?}, got {got:?}"),
    }
}

/// Execute a WASM function with parameters
///
/// # Arguments
/// * `component` - The loaded component
/// * `engine` - The wasmtime engine
/// * `function_name` - Name of the function to execute (supports "interface#function" format)
/// * `params` - WASM values to pass as parameters
///
/// # Returns
/// The result of the function execution
fn execute_function(
    component: &Component,
    engine: &Engine,
    function_name: &str,
    params_json: Vec<serde_json::Value>,
) -> Result<Vec<Val>> {
    let mut store = Store::new(engine, MyState::default());
    let mut linker = Linker::new(engine);

    // Add WASI to the linker
    add_to_linker_sync(&mut linker).context("Failed to link WASI")?;

    // Instantiate the component
    let instance = linker
        .instantiate(&mut store, component)
        .context("Failed to instantiate component")?;
    // Parse function name to support "interface#function" format
    let func = find_func(&instance, &mut store, function_name).context("find func")?;

    let (params, mut results) = {
        let sig = func.ty(&store);

        let expect = sig.params();
        let mut params = Vec::with_capacity(expect.len());
        for ((name, t), v) in expect.zip(params_json.into_iter()) {
            let w = json_to_wasm_value(v, &t).with_context(|| format!("bad param {name}"))?;
            params.push(w);
        }

        let results = vec![Val::Bool(false); sig.results().len()];

        (params, results)
    };

    func.call(&mut store, &params, &mut results)
        .with_context(|| format!("Failed to call function '{}'", function_name))?;

    Ok(results)
}

/// Convert WASM value to JSON value
///
/// # Arguments
/// * `val` - WASM value to convert
///
/// # Returns
/// JSON representation of the value
fn wasm_value_to_json(val: &Val) -> serde_json::Value {
    match val {
        Val::Bool(b) => serde_json::Value::Bool(*b),
        Val::U8(u) => serde_json::Value::Number(serde_json::Number::from(*u)),
        Val::S32(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
        Val::U32(u) => serde_json::Value::Number(serde_json::Number::from(*u)),
        Val::S64(i) => serde_json::json!(*i),
        Val::U64(u) => serde_json::json!(*u),
        Val::Float32(f) => serde_json::Value::Number(
            serde_json::Number::from_f64(*f as f64).unwrap_or(serde_json::Number::from(0)),
        ),
        Val::Float64(f) => serde_json::Value::Number(
            serde_json::Number::from_f64(*f).unwrap_or(serde_json::Number::from(0)),
        ),
        Val::Char(c) => serde_json::json!(c.to_string()),
        Val::String(s) => serde_json::Value::String(s.clone()),
        Val::List(items) => {
            let json_items: Vec<serde_json::Value> = items.iter().map(wasm_value_to_json).collect();
            serde_json::json!(json_items)
        }
        Val::Result(result) => match result.as_ref() {
            Ok(Some(ok_val)) => serde_json::json!({ "ok": wasm_value_to_json(ok_val) }),
            Ok(None) => serde_json::json!("_"),
            Err(Some(err_val)) => serde_json::json!({ "err": wasm_value_to_json(err_val) }),
            Err(None) => serde_json::json!("_"),
        },
        _ => serde_json::json!(format!("{:?}", val)),
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Run with error handling
    run(args).context("run")
}

fn run(args: Args) -> Result<()> {
    println!("WASM file: {:?}", args.wasm);
    println!("Function: {}", args.function);
    println!("Parameters: {:?}", args.params);

    // Parse JSON parameters from command line arguments
    let params = parse_json_params(&args.params).context("parse params")?;
    println!("Parsed parameters: {:?}", params);

    // Load WASM component file (must be a pre-packaged WASM component)
    let (component, engine) = load_component(&args.wasm)?;
    println!("Successfully loaded WASM component");

    // Execute the function
    let result =
        execute_function(&component, &engine, &args.function, params).context("execute")?;

    // Convert WASM results to JSON
    let json_results: Vec<serde_json::Value> = result.iter().map(wasm_value_to_json).collect();

    // Create output
    let out = if json_results.len() == 1 {
        json_results[0].clone()
    } else {
        serde_json::json!(json_results)
    };

    println!("{}", serde_json::to_string(&out)?);

    Ok(())
}

fn find_func<T>(i: &Instance, mut store: &mut Store<T>, name: &str) -> Result<Func> {
    let (interface, func_name) = name
        .split_once('#')
        .ok_or_else(|| anyhow::anyhow!("must in form of 'interface#func'"))?;

    let ii = i
        .get_export_index(&mut *store, None, interface)
        .ok_or_else(|| anyhow::anyhow!("miss interface: {interface}"))?;

    let fi = i
        .get_export_index(&mut store, Some(&ii), func_name)
        .with_context(|| format!("miss export-index for func '{func_name}'"))?;

    i.get_func(&mut store, fi)
        .with_context(|| format!("miss func '{func_name}'"))
}
