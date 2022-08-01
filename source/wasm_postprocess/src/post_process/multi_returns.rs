// This code is heavily inspired by wasm-multi-value-reverse-polyfil code
// https://github.com/vmx/wasm-multi-value-reverse-polyfill
use crate::PostProcessError;
use colored::Colorize;
use std::{env, fs, path::PathBuf, process};
use walrus::{ExportId, ExportItem, FunctionId, Module, ValType};

pub fn make_wasm_have_multiple_returns(
    src_path: PathBuf,
    output_path: Option<PathBuf>,
) -> Result<PathBuf, PostProcessError> {
    let init_arg = String::from("init i32 i32");
    let query_arg = String::from("query i32 i32");
    let execute = String::from("execute i32 i32");

    let args = vec![init_arg, query_arg, execute];

    let output_path =
        write_out_wasm_returning_multi_value(src_path, output_path, args)?;

    Ok(output_path)
}

fn get_val_type(raw_type: &&str) -> Result<ValType, PostProcessError> {
    match *raw_type {
        "i32" => Ok(ValType::I32),
        "i64" => Ok(ValType::I64),
        "f32" => Ok(ValType::F32),
        "f64" => Ok(ValType::F64),
        _ => {
            return Err(format!(
                "unknown return type `{}`. It must be one of \
                i32 | i64 | f32 | f64.",
                raw_type
            )
            .into());
        }
    }
}

/// Returns the transformations.
///
/// The transformations are a list with a tuple containing the return types.
///
/// The input parameters are expected to be a list of parameters, each of
/// them having the form:
///
///     return_value_type_1 return_value_type_2 return_value_type_n
///
/// Each separate by whitespace.
fn parse_args(
    args: &[String],
) -> Result<Vec<(String, Vec<ValType>)>, PostProcessError> {
    let mut transformations = vec![];

    for raw_input in args.iter() {
        let mut input_split: Vec<&str> = raw_input.split_whitespace().collect();

        let function_name = input_split.remove(0).to_string();

        let val_types = input_split
            .iter()
            .map(|input| {
                let val_type = get_val_type(&input)?;
                Ok(val_type)
            })
            .collect::<Result<Vec<ValType>, PostProcessError>>()?;

        if val_types.len() < 2 {
            return Err(format!(
                "there must be at least two return types for function `{}`, \
                else it's not a multi-value return",
                function_name,
            )
            .into());
        }

        transformations.push((function_name, val_types));
    }

    Ok(transformations)
}

// /// Returns the export and function IDs.
fn get_ids_by_name(
    module: &Module,
    function_name: &str,
) -> Result<(ExportId, FunctionId), PostProcessError> {
    let export = module
        .exports
        .iter()
        .find(|&exp| exp.name == function_name)
        .ok_or(format!(
            "cannot find function with name `{}`",
            function_name
        ))?;

    match export.item {
        ExportItem::Function(function_id) => Ok((export.id(), function_id)),
        _ => panic!("item is not a function"),
    }
}

fn write_out_wasm_returning_multi_value(
    src_path: PathBuf,
    output_path: Option<PathBuf>,
    args: Vec<String>,
) -> Result<PathBuf, PostProcessError> {
    println!(
        "[wasm_postprocess] Start processing file: {}",
        src_path.to_string_lossy(),
    );

    let transformations = parse_args(&args)?;

    let wasm = match wit_text::parse_file(&src_path) {
        Ok(w) => w,
        Err(err) => {
            return Err(format!(
                "input file cannot be parsed, path: {:?}, err: {}",
                src_path, err
            )
            .into());
        }
    };

    match wit_validator::validate(&wasm) {
        Ok(_) => (),
        Err(err) => {
            return Err(format!("input wasm is invalid, err: {}", err).into());
        }
    };

    let mut module = match walrus::ModuleConfig::new()
        .strict_validate(false)
        .on_parse(wit_walrus::on_parse)
        .parse(&wasm)
    {
        Ok(m) => m,
        Err(err) => {
            return Err(format!(
                "Cannot parse input file as proper wasm, err: {}",
                err
            )
            .into());
        }
    };

    let shadow_stack_pointer =
        wasm_bindgen_wasm_conventions::get_shadow_stack_pointer(&module)
            .ok_or("cannot get shadow stack pointer")?;

    let memory = wasm_bindgen_wasm_conventions::get_memory(&module)?;

    let to_xform =
        transformations
            .iter()
            .map(|(function_name, result_types)| {
                println!(
                    "[postprocess] Make `{}` function return `{:?}`.",
                    function_name, result_types
                );

                let (_export_id, function_id) =
                    get_ids_by_name(&module, function_name)?;

                Ok((function_id, 0, result_types.to_vec()))
            })
            .collect::<Result<
                Vec<(FunctionId, usize, Vec<ValType>)>,
                PostProcessError>>()?;

    let export_ids: Vec<ExportId> = transformations
        .iter()
        .map(|(function_name, _)| {
            let (export_id, _function_id) =
                get_ids_by_name(&module, function_name)?;

            Ok(export_id)
        })
        .collect::<Result<Vec<ExportId>, PostProcessError>>()?;

    let wrappers = wasm_bindgen_multi_value_xform::run(
        &mut module,
        memory,
        shadow_stack_pointer,
        &to_xform[..],
    )?;

    for (export_id, id) in export_ids.into_iter().zip(wrappers) {
        let mut_export = module.exports.get_mut(export_id);
        mut_export.item = id.into();
    }

    let output_bytes = module.emit_wasm();

    let opath = match output_path {
        Some(p) => p,
        None => {
            let stem = src_path
                .file_stem()
                .ok_or("file name should exist")?
                .to_str()
                .ok_or("file name should be stringified")?
                .to_owned();

            let mut p = src_path;
            p.pop();
            p.join(format!("{}.postprocess.wasm", stem))
        }
    };

    println!(
        "[wasm_postprocess] writing output to path: {}",
        opath.to_string_lossy().yellow()
    );

    fs::write(&opath, output_bytes)?;

    Ok(opath)
}
