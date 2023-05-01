use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetTriple,
};
use inkwell::values::BasicValueEnum;
use inkwell::OptimizationLevel;
use std::path::Path;
use std::process::Command;

// Define the AST
pub enum Expr {
    Int(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

// Define a function to generate LLVM IR for each AST node:
fn codegen_expr<'ctx>(
    context: &'ctx Context,
    builder: &Builder<'ctx>,
    expr: &Expr,
) -> BasicValueEnum<'ctx> {
    let i64_type = context.i64_type();

    match expr {
        Expr::Int(value) => {
            let int_value = i64_type.const_int(*value as u64, true);
            BasicValueEnum::IntValue(int_value)
        }
        Expr::Add(lhs, rhs) => {
            let lhs_value = codegen_expr(context, builder, lhs);
            let rhs_value = codegen_expr(context, builder, rhs);
            let result = builder.build_int_add(
                lhs_value.into_int_value(),
                rhs_value.into_int_value(),
                "add",
            );
            BasicValueEnum::IntValue(result)
        }
        Expr::Mul(lhs, rhs) => {
            let lhs_value = codegen_expr(context, builder, lhs);
            let rhs_value = codegen_expr(context, builder, rhs);
            let result = builder.build_int_mul(
                lhs_value.into_int_value(),
                rhs_value.into_int_value(),
                "mul",
            );
            BasicValueEnum::IntValue(result)
        }
        // Similarly, implement Sub, Mul, and Div
        // ...
        _ => unimplemented!(),
    }
}

// fn codegen_expr<'ctx>(builder: &Builder<'ctx>, expr: &Expr) -> BasicValueEnum<'ctx> {
//     let i64_type = builder.get_context().i64_type();
//     // let i64_type = builder.get_context().i64_type();

//     match expr {
//         Expr::Int(value) => {
//             let int_value = i64_type.const_int(*value as u64, true);
//             BasicValueEnum::IntValue(int_value)
//         }
//         Expr::Add(lhs, rhs) => {
//             let lhs_value = codegen_expr(builder, lhs);
//             let rhs_value = codegen_expr(builder, rhs);
//             let result = builder.build_int_add(
//                 lhs_value.into_int_value(),
//                 rhs_value.into_int_value(),
//                 "add",
//             );
//             BasicValueEnum::IntValue(result)
//         }
//         // Similarly, implement Sub, Mul, and Div
//         // ...
//         _ => unimplemented!(),
//     }
// }

// Generate the LLVM IR for the entire AST
fn codegen_module<'ctx>(context: &'ctx Context, module: &Module<'ctx>, expr: &Expr) {
    let i64_type = context.i64_type();
    let fn_type = i64_type.fn_type(&[], false);
    let function = module.add_function("main", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();

    builder.position_at_end(basic_block);
    let result = codegen_expr(&context, &builder, expr);
    builder.build_return(Some(&result));

    // Verify the generated module for correctness
    if module.verify().is_err() {
        eprintln!("Error: {:?}", module);
        std::process::exit(1);
    }
}

fn main() {
    // Create a code generation context and module
    let context = Context::create();
    let module = context.create_module("calc");

    // Call the codegen_module function in main
    let ast = Expr::Add(
        Box::new(Expr::Int(5)),
        Box::new(Expr::Mul(Box::new(Expr::Int(3)), Box::new(Expr::Int(4)))),
    );
    codegen_module(&context, &module, &ast);

    // Initialize the target
    Target::initialize_native(&InitializationConfig::default()).unwrap();
    // let target_triple = context.target_triple();
    let triple = TargetTriple::create("aarch64-apple-darwin");
    // let triple = TargetTriple::create("x86_64-pc-linux-gnu");
    // let triple = TargetTriple::create("x86_64-pc-linux-gnu");
    let target = Target::from_triple(&triple).unwrap();

    // let target_triple = context.target_triple();
    // let target = Target::from_triple(&target_triple).unwrap();
    let target_machine = target
        .create_target_machine(
            &triple,
            "generic",
            "",
            OptimizationLevel::None,
            RelocMode::Default,
            CodeModel::Default,
        )
        .expect("Could not create target machine");

    let target_data = target_machine.get_target_data();
    module.set_data_layout(&target_data.get_data_layout());
    module.set_triple(&triple);

    let object_file_name = "calc.o";
    target_machine
        .write_to_file(&module, FileType::Object, Path::new(object_file_name))
        .unwrap();

    // Link the object file to create an executable
    let linker = if cfg!(target_os = "windows") {
        "link.exe"
    } else {
        "gcc"
    };

    let output_file_name = if cfg!(target_os = "windows") {
        "calc.exe"
    } else {
        "calc"
    };

    let status = Command::new(linker)
        .arg(object_file_name)
        .arg("-o")
        .arg(output_file_name)
        .status()
        .unwrap();

    if !status.success() {
        eprintln!("Error: Failed to link the object file");
        std::process::exit(1);
    }

    // Clean up the object file
    // std::fs::remove_file(object_file_name).unwrap();

    println!(
        "Successfully compiled and linked the executable: {}",
        output_file_name
    );
}
