use inkwell::{
    context::Context, passes::PassManager, targets::{FileType, InitializationConfig, RelocMode, Target, TargetMachine}, types::BasicType, OptimizationLevel
};

use std::{collections::HashMap, process::Command};

mod var_collecter;

use crate::ir::definition;
use crate::semantic_analysis::typecheck::SymbolTable;

pub struct LLVMGenerator<'a> {
    symbol_table: HashMap<String, inkwell::values::PointerValue<'a>>,
    label_table: HashMap<String, inkwell::basic_block::BasicBlock<'a>>,
    context: &'a inkwell::context::Context,
    module: inkwell::module::Module<'a>,
    current_function: String,
    frontend_symbol_table: &'a SymbolTable,
}

fn __ty_to_llvm_ty<'a>(ctx: &'a inkwell::context::Context, ty: &definition::Type) -> inkwell::types::BasicTypeEnum<'a> {
    match ty {
        definition::Type::I32 => ctx.i32_type().as_basic_type_enum(),
        definition::Type::Pointer(box inner_ty) => {
            let inner_ty = __ty_to_llvm_ty(ctx, inner_ty);
            inner_ty.ptr_type(inkwell::AddressSpace::from(0)).as_basic_type_enum()
        }
        definition::Type::Function(_, _) => unreachable!(),
    }
}

#[allow(dead_code)]
pub fn sizeof_type(t: &definition::Type) -> u64 {
    Target::initialize_all(&InitializationConfig::default());
    let target = Target::from_triple(&TargetMachine::get_default_triple()).unwrap();
    let target_machine = target
        .create_target_machine(
            &TargetMachine::get_default_triple(),
            "generic",
            "",
            OptimizationLevel::Aggressive,
            RelocMode::Default,
            inkwell::targets::CodeModel::Default,
        )
        .unwrap();
    let target_data = target_machine.get_target_data();
    // generate a ctx temporarily
    let ctx = Context::create();
    target_data.get_abi_size(&__ty_to_llvm_ty(&ctx, t))
}

impl<'a> LLVMGenerator<'a> {
    pub fn create_context() -> Context {
        Context::create()
    }

    pub fn new(context: &'a Context, frontend_symbol_table: &'a SymbolTable) -> Self {
        let module = context.create_module("main");
        Self {
            symbol_table: HashMap::new(),
            label_table: HashMap::new(),
            context,
            module,
            current_function: String::new(),
            frontend_symbol_table,
        }
    }

    pub fn generate(mut self, program: definition::Program, output_file: &str) {
        Target::initialize_all(&InitializationConfig::default());

        for f in program.functions {
            self.generate_function(f) 
        }
    
        // Set up the target machine for the host
        let target = Target::from_triple(&TargetMachine::get_default_triple()).unwrap();
        let target_machine = target
            .create_target_machine(
                &TargetMachine::get_default_triple(),
                "generic",
                "",
                OptimizationLevel::Aggressive,
                RelocMode::Default,
                inkwell::targets::CodeModel::Default,
            )
            .unwrap();

        // Print out the generated IR
        self.module.print_to_file("output.ll").expect("Failed to print module to file");

        match self.module.verify() {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Error verifying module: {}", err.to_string());
                return;
            }
        }

        // compile to assembly
        target_machine
            .write_to_file(
                &self.module,
                FileType::Assembly,
                std::path::Path::new("output.s"),
            ).expect("uh oh");


        // Compile to an object file
        let obj_file = "output.o";
        target_machine
            .write_to_file(&self.module, FileType::Object, std::path::Path::new(obj_file))
            .expect("Failed to generate object file");
    
        // Use clang to link and create an executable
        let output = Command::new("clang")
            .args(&[obj_file, "-o", output_file])
            .output()
            .expect("Failed to run clang");
    
        // remove the object file
        std::fs::remove_file(obj_file).expect("Failed to remove object file");
    
        if !output.status.success() {
            eprintln!(
                "Error while linking with clang: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            return;
        }
    
        println!("Executable generated: ./{}", output_file);
    }

    fn ty_to_llvm_ty(&self, ty: &definition::Type) -> inkwell::types::BasicTypeEnum<'a> {
        match ty {
            definition::Type::I32 => self.context.i32_type().as_basic_type_enum(),
            /*definition::Type::I64 => self.context.i64_type().as_basic_type_enum(),
            definition::Type::U32 => self.context.i32_type().as_basic_type_enum(),
            definition::Type::U64 => self.context.i64_type().as_basic_type_enum(),
            definition::Type::F64 => self.context.f64_type().as_basic_type_enum(),
            definition::Type::Box(box inner_ty) |*/
            definition::Type::Pointer(box inner_ty) => {
                let inner_ty = self.ty_to_llvm_ty(inner_ty);
                inner_ty.ptr_type(inkwell::AddressSpace::from(0)).as_basic_type_enum()
            }
            definition::Type::Function(_, _) => unreachable!(),
        }
    }

    fn get_metadata_type(&self, ty: &definition::Type) -> inkwell::types::BasicMetadataTypeEnum<'a> {
        match ty {
            definition::Type::I32 => inkwell::types::BasicMetadataTypeEnum::IntType(self.context.i32_type()),
            /*definition::Type::I64 => inkwell::types::BasicMetadataTypeEnum::IntType(self.context.i64_type()),
            definition::Type::U32 => inkwell::types::BasicMetadataTypeEnum::IntType(self.context.i32_type()),
            definition::Type::U64 => inkwell::types::BasicMetadataTypeEnum::IntType(self.context.i64_type()),
            definition::Type::F64 => inkwell::types::BasicMetadataTypeEnum::FloatType(self.context.f64_type()),*/

            /*definition::Type::Box(box inner_ty) |*/
            definition::Type::Pointer(box inner_ty) => {
                // turn the inner_ty into a inkwell::types::PointerType
                let inner_ty = self.ty_to_llvm_ty(inner_ty);
                inkwell::types::BasicMetadataTypeEnum::PointerType(inner_ty.ptr_type(inkwell::AddressSpace::from(0)))
            }

            definition::Type::Function(_, _) => unreachable!(),
        }
    }

    fn get_ptr_from_val(&mut self, val: definition::Val) -> inkwell::values::PointerValue<'a> {
        match val {
            definition::Val::Number(_) => {
                panic!("uh oh")
            }
            definition::Val::Var(name) => {
                // lookup the variable
                if let Some(ptr_val) = self.symbol_table.get(&name) {
                    *ptr_val
                } else {
                    panic!("Variable not found")
                }
            }
        }
    }

    fn generate_function(&mut self, ir_function: definition::Function) {
        let mut var_collector = var_collecter::Collector::new(&self.frontend_symbol_table);
        var_collector.collect_function(&ir_function);
        let variables = var_collector.variables;

        let (function, ret_ty) = match self.module.get_function(&ir_function.name) {
            Some(f) => (f, f.get_type().get_return_type().unwrap()),
            None => {
                let param_types = ir_function.params.iter().map(|(_, ty)| ty).collect::<Vec<_>>();
                let llvm_ret_type = self.ty_to_llvm_ty(&ir_function.return_type);

                let linkage = if true { // check if global (automatically rn)
                    inkwell::module::Linkage::External
                } else {
                    inkwell::module::Linkage::ExternalWeak
                };

                let param_types = param_types.iter().map(|ty| self.get_metadata_type(ty)).collect::<Vec<_>>();

                let fn_type = llvm_ret_type.fn_type(&param_types, false);
                (self.module.add_function(&ir_function.name, fn_type, Some(linkage)), llvm_ret_type)
            }
        };

        function.set_linkage(if true { // if global
            inkwell::module::Linkage::External
        } else {
            inkwell::module::Linkage::ExternalWeak
        });
        
        let entry = self.context.append_basic_block(function, "entry");
        let builder = self.context.create_builder();
        self.current_function = ir_function.name;
        self.symbol_table.clear();

        // self.label_table.clear();
        builder.position_at_end(entry);

        // allocate space for variables
        for (variable, ty) in variables {
            let ty = self.ty_to_llvm_ty(&ty);

            let ptr_val = builder.build_alloca(ty, &variable).expect("uh oh");
            self.symbol_table.insert(variable, ptr_val);
        }

        function.get_params().into_iter();

        for (param, (name, _)) in function.get_params().into_iter().zip(ir_function.params.into_iter()) {
            let ptr_val = builder.build_alloca(param.get_type(), name.as_str()).expect("uh oh");
            builder.build_store(ptr_val, param).expect("uh oh");
            self.symbol_table.insert(name, ptr_val);
        }

        // generate instructions
        for instruction in ir_function.body {
            self.generate_instruction(&builder, instruction);
        }

        // return 0 (in case we end with a label)
        builder.build_return(Some(&ret_ty.const_zero())).expect("uh oh");

        let fpm = PassManager::create(&self.module);

        //fpm.add_instruction_combining_pass();
        //fpm.add_reassociate_pass();
        //fpm.add_gvn_pass();
        //fpm.add_cfg_simplification_pass();
        //fpm.add_basic_alias_analysis_pass();
        //fpm.add_promote_memory_to_register_pass();

        fpm.initialize();

        fpm.run_on(&function);

        fpm.finalize();
    }

    fn generate_instruction(&mut self, builder: &inkwell::builder::Builder<'a>, instruction: definition::Instruction, ) {
        match instruction {
            definition::Instruction::Return(val) => {
                let return_val = self.val_to_base(val, builder);
                builder.build_return(Some(&return_val)).expect("uh oh");
                let temp_label = self.context.append_basic_block(self.module.get_function(&self.current_function).unwrap(), "after term");
                builder.position_at_end(temp_label);
            }
            definition::Instruction::Binary { op, src1, src2, dst } => {
                let (is_unsigned, is_double) = (false, false);

                let src1_val = self.val_to_base(src1, builder);
                let src2_val = self.val_to_base(src2, builder);
                let dest_val = self.get_ptr_from_val(dst);

                match op {
                    definition::Binop::Add => {
                        let result = if !is_double {
                            inkwell::values::BasicValueEnum::IntValue(builder.build_int_add(src1_val.into_int_value(), src2_val.into_int_value(), "add").expect("uh oh"))
                        } else {
                            inkwell::values::BasicValueEnum::FloatValue(builder.build_float_add(src1_val.into_float_value(), src2_val.into_float_value(), "add").expect("uh oh"))
                        };
                        builder.build_store(dest_val, result).expect("uh oh");
                    }
                    definition::Binop::Sub => {
                        let result = if !is_double {
                            inkwell::values::BasicValueEnum::IntValue(builder.build_int_sub(src1_val.into_int_value(), src2_val.into_int_value(), "subtract").expect("uh oh"))
                        } else {
                            inkwell::values::BasicValueEnum::FloatValue(builder.build_float_sub(src1_val.into_float_value(), src2_val.into_float_value(), "subtract").expect("uh oh"))
                        };
                        builder.build_store(dest_val, result).expect("uh oh");
                    }
                    definition::Binop::Mul => {
                        let result = if !is_double {
                            inkwell::values::BasicValueEnum::IntValue(builder.build_int_mul(src1_val.into_int_value(), src2_val.into_int_value(), "multiply").expect("uh oh"))
                        } else {
                            inkwell::values::BasicValueEnum::FloatValue(builder.build_float_mul(src1_val.into_float_value(), src2_val.into_float_value(), "multiply").expect("uh oh"))
                        };
                        builder.build_store(dest_val, result).expect("uh oh");
                    }
                    definition::Binop::Div => {
                        let result = if is_unsigned {
                            inkwell::values::BasicValueEnum::IntValue(builder.build_int_signed_div(src1_val.into_int_value(), src2_val.into_int_value(), "divide").expect("uh oh"))
                        } else {
                            if !is_double {
                                inkwell::values::BasicValueEnum::IntValue(builder.build_int_unsigned_div(src1_val.into_int_value(), src2_val.into_int_value(), "divide").expect("uh oh"))
                            } else {
                                inkwell::values::BasicValueEnum::FloatValue(builder.build_float_div(src1_val.into_float_value(), src2_val.into_float_value(), "divide").expect("uh oh"))
                            }
                        };
                        builder.build_store(dest_val, result).expect("uh oh");
                    }
                    definition::Binop::Mod => {
                        let result = if is_unsigned {
                            inkwell::values::BasicValueEnum::IntValue(builder.build_int_signed_rem(src1_val.into_int_value(), src2_val.into_int_value(), "mod").expect("uh oh"))
                        } else {
                            if !is_double {
                                inkwell::values::BasicValueEnum::IntValue(builder.build_int_unsigned_rem(src1_val.into_int_value(), src2_val.into_int_value(), "mod").expect("uh oh"))
                            } else {
                                inkwell::values::BasicValueEnum::FloatValue(builder.build_float_rem(src1_val.into_float_value(), src2_val.into_float_value(), "mod").expect("uh oh"))
                            }
                        };
                        builder.build_store(dest_val, result).expect("uh oh");
                    }
                    definition::Binop::Equal => {
                        let result = builder.build_int_compare(inkwell::IntPredicate::EQ, src1_val.into_int_value(), src2_val.into_int_value(), "equal").expect("uh oh");
                        let result = builder.build_int_z_extend(result, self.context.i32_type(), "extend").expect("uh oh");
                        builder.build_store(dest_val, result).expect("uh oh");
                    }
                }
            }
            definition::Instruction::Copy { src, dst } => {
                let src_val = self.val_to_base(src, builder);
                let dest_val = self.get_ptr_from_val(dst);
                builder.build_store(dest_val, src_val).expect("uh oh");
            }
            definition::Instruction::Jump(label) => {
                let block = self.get_block(&label);
                builder.build_unconditional_branch(block).expect("uh oh");
                let temp_label = self.context.append_basic_block(self.module.get_function(&self.current_function).unwrap(), "after term");
                builder.position_at_end(temp_label);
            }
            definition::Instruction::JumpIfZero(val, label) => {
                let val = self.val_to_base(val, builder);
                let block = self.get_block(&label);
                // convert to i1
                let val = builder.build_int_compare(inkwell::IntPredicate::EQ, val.into_int_value(), self.context.i32_type().const_zero(), "compare").expect("uh oh");
                let temp_label = self.context.append_basic_block(self.module.get_function(&self.current_function).unwrap(), "no branch");
                builder.build_conditional_branch(val, block, temp_label).expect("uh oh");
                builder.position_at_end(temp_label);
            }
            definition::Instruction::JumpIfNotZero(val, label) => {
                let val = self.val_to_base(val, builder);
                let block = self.get_block(&label);
                // convert to i1
                let val = builder.build_int_compare(inkwell::IntPredicate::NE, val.into_int_value(), self.context.i32_type().const_zero(), "compare").expect("uh oh");
                let temp_label = self.context.append_basic_block(self.module.get_function(&self.current_function).unwrap(), "no branch");
                builder.build_conditional_branch(val, temp_label, block).expect("uh oh");
                builder.position_at_end(temp_label);
            }
            definition::Instruction::Label(label) => {
                // jump to the label, since every block needs to end with some terminator
                let block = self.get_block(&label);
                    builder.build_unconditional_branch(block).expect("uh oh");
                builder.position_at_end(block);
            }
            definition::Instruction::FunctionCall(name, args, dst) => {
                let function = match self.module.get_function(&name) {
                    Some(f) => f,
                    None => {
                        let entry = self.frontend_symbol_table.get(&name).expect("Function not found");
                        let (param_types, ret_type) = match &entry.ty {
                            definition::Type::Function(params, ret) => (params, ret),
                            _ => unreachable!("uh oh")
                        };
                        let linkage = if true { // is global
                            inkwell::module::Linkage::External
                        } else {
                            inkwell::module::Linkage::ExternalWeak
                        };

                        let param_types = param_types.iter().map(|ty| self.get_metadata_type(ty)).collect::<Vec<_>>();

                        let ret_type = self.ty_to_llvm_ty(&ret_type);

                        let fn_type = ret_type.fn_type(&param_types, false);

                        self.module.add_function(&name, fn_type, Some(linkage))
                    }
                };
                let mut arg_vals = Vec::new();
                for arg in args {
                    arg_vals.push(self.val_to_base(arg, builder).into())
                }
                let dest_val = self.get_ptr_from_val(dst);
                let result = builder.build_call(function, &arg_vals, "call").expect("uh oh");
                builder.build_store(dest_val, result.try_as_basic_value().left().unwrap()).expect("uh oh");
            }
            definition::Instruction::GetAddress(src, dest, ..) => {
                // get address of src and store it in dest
                let ptr = *match src {
                    definition::Val::Var(name) => self.symbol_table.get(&name).expect("Variable not found"),
                    definition::Val::Number(_) => panic!("uh oh")
                };
                let dest_val = self.get_ptr_from_val(dest);

                builder.build_store(dest_val, ptr).expect("uh oh");
            }
            definition::Instruction::Load(src_ptr, dest) => {
                let src_ptr_val = self.val_to_base(src_ptr, builder);
                let dest_val = self.get_ptr_from_val(dest);
                let result = builder.build_load(src_ptr_val.into_pointer_value(), "load").expect("uh oh");
                builder.build_store(dest_val, result).expect("uh oh");
            }
            definition::Instruction::Store(src, dest_ptr) => {
                println!("{:?} <- {:?}", dest_ptr, src);
                let src_val = self.val_to_base(src, builder);
                let dest_ptr_val = self.val_to_base(dest_ptr, builder);
                builder.build_store(dest_ptr_val.into_pointer_value(), src_val).expect("uh oh");
            }
            definition::Instruction::AddPtr { ptr, index, dst } => {
                let ptr_val = self.val_to_base(ptr.clone(), builder);
                let index_val = self.val_to_base(index, builder);
                let dest_val = self.get_ptr_from_val(dst); 
                
                let ptr_val = ptr_val.into_pointer_value();

                println!("{:?}", ptr);

                let result = unsafe { builder.build_gep(ptr_val, &[index_val.into_int_value()], "addptr").expect("uh oh") };
                builder.build_store(dest_val, result).expect("uh oh");
            }
        }
    }

    fn get_block(&mut self, label: &String) -> inkwell::basic_block::BasicBlock<'a> {
        if let Some(block) = self.label_table.get(label) {
            *block
        } else {
            let block = self.context.append_basic_block(self.module.get_function(&self.current_function).unwrap(), &label);
            self.label_table.insert(label.clone(), block);
            block
        }
    }

    fn val_to_base(&self, val: definition::Val, builder: &inkwell::builder::Builder<'a>) -> inkwell::values::BasicValueEnum<'a> {
        let i32_type = self.context.i32_type();
        let _i64_type = self.context.i64_type();

        match val {
            definition::Val::Number(value) => {
                let ty = i32_type;

                inkwell::values::BasicValueEnum::IntValue(ty.const_int(value, true))
            }
            definition::Val::Var(name) => {
                // lookup the variable
                let ptr_val = self.symbol_table.get(&name).expect("Variable not found");

                builder.build_load(*ptr_val, &name).expect("uh oh")
            }
        }
    }
}