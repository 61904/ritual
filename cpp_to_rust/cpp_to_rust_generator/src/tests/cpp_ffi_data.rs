use caption_strategy::*;
use cpp_ffi_data::*;
use cpp_function::{CppFunctionKind, ReturnValueAllocationPlace};
use cpp_operator::CppOperator;
use cpp_type::*;
use tests::cpp_method::{empty_membership, empty_regular_method};

#[test]
fn argument_meaning() {
  let a1 = CppFfiArgumentMeaning::This;
  assert!(!a1.is_argument());

  let a2 = CppFfiArgumentMeaning::Argument(2);
  assert!(a2.is_argument());

  let a3 = CppFfiArgumentMeaning::ReturnValue;
  assert!(!a3.is_argument());
}

#[test]
fn argument_int() {
  let arg = CppFfiFunctionArgument {
    name: "arg1".to_string(),
    argument_type: CppFfiType {
      original_type: CppType {
        indirection: CppTypeIndirection::None,
        is_const: false,
        is_const2: false,
        base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Int),
      },
      ffi_type: CppType {
        indirection: CppTypeIndirection::None,
        is_const: false,
        is_const2: false,
        base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Int),
      },
      conversion: CppTypeConversionToFfi::NoChange,
    },
    meaning: CppFfiArgumentMeaning::Argument(0),
  };
  assert_eq!(
    arg.caption(ArgumentCaptionStrategy::NameOnly).unwrap(),
    "arg1"
  );
  assert_eq!(
    arg
      .caption(ArgumentCaptionStrategy::TypeOnly(
        TypeCaptionStrategy::Short,
      ))
      .unwrap(),
    "int"
  );
  assert_eq!(
    arg
      .caption(ArgumentCaptionStrategy::TypeOnly(TypeCaptionStrategy::Full))
      .unwrap(),
    "int"
  );
  assert_eq!(
    arg
      .caption(ArgumentCaptionStrategy::TypeAndName(
        TypeCaptionStrategy::Short,
      ))
      .unwrap(),
    "int_arg1"
  );
  assert_eq!(
    arg
      .caption(ArgumentCaptionStrategy::TypeAndName(
        TypeCaptionStrategy::Full,
      ))
      .unwrap(),
    "int_arg1"
  );
  assert_eq!(arg.to_cpp_code().unwrap(), "int arg1");
}

#[test]
fn argument_int_ptr() {
  let arg = CppFfiFunctionArgument {
    name: "arg1".to_string(),
    argument_type: CppFfiType {
      original_type: CppType {
        indirection: CppTypeIndirection::Ptr,
        is_const: false,
        is_const2: false,
        base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Int),
      },
      ffi_type: CppType {
        indirection: CppTypeIndirection::Ptr,
        is_const: false,
        is_const2: false,
        base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Int),
      },
      conversion: CppTypeConversionToFfi::NoChange,
    },
    meaning: CppFfiArgumentMeaning::Argument(0),
  };
  assert_eq!(
    arg.caption(ArgumentCaptionStrategy::NameOnly).unwrap(),
    "arg1"
  );
  assert_eq!(
    arg
      .caption(ArgumentCaptionStrategy::TypeOnly(
        TypeCaptionStrategy::Short,
      ))
      .unwrap(),
    "int"
  );
  assert_eq!(
    arg
      .caption(ArgumentCaptionStrategy::TypeOnly(TypeCaptionStrategy::Full))
      .unwrap(),
    "int_ptr"
  );
  assert_eq!(
    arg
      .caption(ArgumentCaptionStrategy::TypeAndName(
        TypeCaptionStrategy::Short,
      ))
      .unwrap(),
    "int_arg1"
  );
  assert_eq!(
    arg
      .caption(ArgumentCaptionStrategy::TypeAndName(
        TypeCaptionStrategy::Full,
      ))
      .unwrap(),
    "int_ptr_arg1"
  );
  assert_eq!(arg.to_cpp_code().unwrap(), "int* arg1");
}

#[test]
fn argument_func() {
  let type1 = CppType {
    is_const: false,
    is_const2: false,
    indirection: CppTypeIndirection::None,
    base: CppTypeBase::FunctionPointer(CppFunctionPointerType {
      allows_variadic_arguments: false,
      return_type: Box::new(CppType {
        indirection: CppTypeIndirection::None,
        is_const: false,
        is_const2: false,
        base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Int),
      }),
      arguments: vec![
        CppType {
          indirection: CppTypeIndirection::None,
          is_const: false,
          is_const2: false,
          base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Int),
        },
        CppType {
          indirection: CppTypeIndirection::Ptr,
          is_const: false,
          is_const2: false,
          base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Bool),
        },
      ],
    }),
  };

  let arg = CppFfiFunctionArgument {
    name: "arg1".to_string(),
    argument_type: CppFfiType {
      original_type: type1.clone(),
      ffi_type: type1.clone(),
      conversion: CppTypeConversionToFfi::NoChange,
    },
    meaning: CppFfiArgumentMeaning::Argument(0),
  };
  assert_eq!(
    arg.caption(ArgumentCaptionStrategy::NameOnly).unwrap(),
    "arg1"
  );
  assert_eq!(
    arg
      .caption(ArgumentCaptionStrategy::TypeOnly(
        TypeCaptionStrategy::Short,
      ))
      .unwrap(),
    "func"
  );
  assert_eq!(
    arg
      .caption(ArgumentCaptionStrategy::TypeOnly(TypeCaptionStrategy::Full))
      .unwrap(),
    "int_func_int_bool_ptr"
  );
  assert_eq!(
    arg
      .caption(ArgumentCaptionStrategy::TypeAndName(
        TypeCaptionStrategy::Short,
      ))
      .unwrap(),
    "func_arg1"
  );
  assert_eq!(
    arg
      .caption(ArgumentCaptionStrategy::TypeAndName(
        TypeCaptionStrategy::Full,
      ))
      .unwrap(),
    "int_func_int_bool_ptr_arg1"
  );
  assert_eq!(arg.to_cpp_code().unwrap(), "int (*arg1)(int, bool*)");
}

#[test]
fn signature_two_numbers() {
  let sig = CppFfiMethodSignature {
    arguments: vec![
      CppFfiFunctionArgument {
        name: "arg1".to_string(),
        argument_type: CppFfiType {
          original_type: CppType {
            indirection: CppTypeIndirection::None,
            is_const: false,
            is_const2: false,
            base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Int),
          },
          ffi_type: CppType {
            indirection: CppTypeIndirection::None,
            is_const: false,
            is_const2: false,
            base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Int),
          },
          conversion: CppTypeConversionToFfi::NoChange,
        },
        meaning: CppFfiArgumentMeaning::Argument(0),
      },
      CppFfiFunctionArgument {
        name: "arg2".to_string(),
        argument_type: CppFfiType {
          original_type: CppType {
            indirection: CppTypeIndirection::None,
            is_const: false,
            is_const2: false,
            base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Double),
          },
          ffi_type: CppType {
            indirection: CppTypeIndirection::None,
            is_const: false,
            is_const2: false,
            base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Double),
          },
          conversion: CppTypeConversionToFfi::NoChange,
        },
        meaning: CppFfiArgumentMeaning::Argument(0),
      },
    ],
    return_type: CppFfiType::void(),
  };

  assert_eq!(
    sig
      .arguments_caption(ArgumentCaptionStrategy::NameOnly)
      .unwrap(),
    "arg1_arg2"
  );
  assert_eq!(
    sig
      .arguments_caption(ArgumentCaptionStrategy::TypeOnly(
        TypeCaptionStrategy::Short,
      ))
      .unwrap(),
    "int_double"
  );
  assert_eq!(
    sig
      .arguments_caption(ArgumentCaptionStrategy::TypeOnly(TypeCaptionStrategy::Full))
      .unwrap(),
    "int_double"
  );
  assert_eq!(
    sig
      .arguments_caption(ArgumentCaptionStrategy::TypeAndName(
        TypeCaptionStrategy::Short,
      ))
      .unwrap(),
    "int_arg1_double_arg2"
  );
  assert_eq!(
    sig
      .arguments_caption(ArgumentCaptionStrategy::TypeAndName(
        TypeCaptionStrategy::Full,
      ))
      .unwrap(),
    "int_arg1_double_arg2"
  );

  assert_eq!(
    sig
      .caption(MethodCaptionStrategy::ArgumentsOnly(
        ArgumentCaptionStrategy::NameOnly,
      ))
      .unwrap(),
    "arg1_arg2"
  );
  assert_eq!(
    sig
      .caption(MethodCaptionStrategy::ArgumentsOnly(
        ArgumentCaptionStrategy::TypeOnly(TypeCaptionStrategy::Short,),
      ))
      .unwrap(),
    "int_double"
  );
  assert_eq!(
    sig
      .caption(MethodCaptionStrategy::ArgumentsOnly(
        ArgumentCaptionStrategy::TypeOnly(TypeCaptionStrategy::Full),
      ))
      .unwrap(),
    "int_double"
  );
  assert_eq!(
    sig
      .caption(MethodCaptionStrategy::ArgumentsOnly(
        ArgumentCaptionStrategy::TypeAndName(TypeCaptionStrategy::Short,),
      ))
      .unwrap(),
    "int_arg1_double_arg2"
  );
  assert_eq!(
    sig
      .caption(MethodCaptionStrategy::ArgumentsOnly(
        ArgumentCaptionStrategy::TypeAndName(TypeCaptionStrategy::Full,),
      ))
      .unwrap(),
    "int_arg1_double_arg2"
  );

  assert_eq!(sig.caption(MethodCaptionStrategy::ConstOnly).unwrap(), "");

  assert!(!sig.has_const_this());
}

#[test]
fn signature_class_method() {
  let sig = CppFfiMethodSignature {
    arguments: vec![
      CppFfiFunctionArgument {
        name: "this_ptr".to_string(),
        argument_type: CppFfiType {
          original_type: CppType {
            indirection: CppTypeIndirection::Ptr,
            is_const: false,
            is_const2: false,
            base: CppTypeBase::Class(CppTypeClassBase {
              name: "Class1".to_string(),
              template_arguments: None,
            }),
          },
          ffi_type: CppType {
            indirection: CppTypeIndirection::Ptr,
            is_const: false,
            is_const2: false,
            base: CppTypeBase::Class(CppTypeClassBase {
              name: "Class1".to_string(),
              template_arguments: None,
            }),
          },
          conversion: CppTypeConversionToFfi::NoChange,
        },
        meaning: CppFfiArgumentMeaning::This,
      },
      CppFfiFunctionArgument {
        name: "arg1".to_string(),
        argument_type: CppFfiType {
          original_type: CppType {
            indirection: CppTypeIndirection::None,
            is_const: false,
            is_const2: false,
            base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Double),
          },
          ffi_type: CppType {
            indirection: CppTypeIndirection::None,
            is_const: false,
            is_const2: false,
            base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Double),
          },
          conversion: CppTypeConversionToFfi::NoChange,
        },
        meaning: CppFfiArgumentMeaning::Argument(0),
      },
    ],
    return_type: CppFfiType::void(),
  };
  assert_eq!(
    sig
      .arguments_caption(ArgumentCaptionStrategy::NameOnly)
      .unwrap(),
    "arg1"
  );
  assert_eq!(
    sig
      .arguments_caption(ArgumentCaptionStrategy::TypeOnly(
        TypeCaptionStrategy::Short,
      ))
      .unwrap(),
    "double"
  );
  assert_eq!(
    sig
      .arguments_caption(ArgumentCaptionStrategy::TypeOnly(TypeCaptionStrategy::Full))
      .unwrap(),
    "double"
  );
  assert_eq!(
    sig
      .arguments_caption(ArgumentCaptionStrategy::TypeAndName(
        TypeCaptionStrategy::Short,
      ))
      .unwrap(),
    "double_arg1"
  );
  assert_eq!(
    sig
      .arguments_caption(ArgumentCaptionStrategy::TypeAndName(
        TypeCaptionStrategy::Full,
      ))
      .unwrap(),
    "double_arg1"
  );

  assert_eq!(sig.caption(MethodCaptionStrategy::ConstOnly).unwrap(), "");

  assert!(!sig.has_const_this());
}

#[test]
fn signature_class_method_const() {
  let sig = CppFfiMethodSignature {
    arguments: vec![
      CppFfiFunctionArgument {
        name: "this_ptr".to_string(),
        argument_type: CppFfiType {
          original_type: CppType {
            indirection: CppTypeIndirection::Ptr,
            is_const: true,
            is_const2: false,
            base: CppTypeBase::Class(CppTypeClassBase {
              name: "Class1".to_string(),
              template_arguments: None,
            }),
          },
          ffi_type: CppType {
            indirection: CppTypeIndirection::Ptr,
            is_const: true,
            is_const2: false,
            base: CppTypeBase::Class(CppTypeClassBase {
              name: "Class1".to_string(),
              template_arguments: None,
            }),
          },
          conversion: CppTypeConversionToFfi::NoChange,
        },
        meaning: CppFfiArgumentMeaning::This,
      },
      CppFfiFunctionArgument {
        name: "arg1".to_string(),
        argument_type: CppFfiType {
          original_type: CppType {
            indirection: CppTypeIndirection::None,
            is_const: false,
            is_const2: false,
            base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Double),
          },
          ffi_type: CppType {
            indirection: CppTypeIndirection::None,
            is_const: false,
            is_const2: false,
            base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Double),
          },
          conversion: CppTypeConversionToFfi::NoChange,
        },
        meaning: CppFfiArgumentMeaning::Argument(0),
      },
    ],
    return_type: CppFfiType::void(),
  };

  assert_eq!(
    sig
      .arguments_caption(ArgumentCaptionStrategy::NameOnly)
      .unwrap(),
    "arg1"
  );
  assert_eq!(
    sig
      .arguments_caption(ArgumentCaptionStrategy::TypeOnly(
        TypeCaptionStrategy::Short,
      ))
      .unwrap(),
    "double"
  );
  assert_eq!(
    sig
      .arguments_caption(ArgumentCaptionStrategy::TypeOnly(TypeCaptionStrategy::Full))
      .unwrap(),
    "double"
  );
  assert_eq!(
    sig
      .arguments_caption(ArgumentCaptionStrategy::TypeAndName(
        TypeCaptionStrategy::Short,
      ))
      .unwrap(),
    "double_arg1"
  );
  assert_eq!(
    sig
      .arguments_caption(ArgumentCaptionStrategy::TypeAndName(
        TypeCaptionStrategy::Full,
      ))
      .unwrap(),
    "double_arg1"
  );

  assert_eq!(
    sig.caption(MethodCaptionStrategy::ConstOnly).unwrap(),
    "const"
  );

  assert_eq!(
    sig
      .caption(MethodCaptionStrategy::ArgumentsOnly(
        ArgumentCaptionStrategy::NameOnly,
      ))
      .unwrap(),
    "arg1"
  );
  assert_eq!(
    sig
      .caption(MethodCaptionStrategy::ArgumentsOnly(
        ArgumentCaptionStrategy::TypeOnly(TypeCaptionStrategy::Short,),
      ))
      .unwrap(),
    "double"
  );
  assert_eq!(
    sig
      .caption(MethodCaptionStrategy::ArgumentsOnly(
        ArgumentCaptionStrategy::TypeOnly(TypeCaptionStrategy::Full),
      ))
      .unwrap(),
    "double"
  );
  assert_eq!(
    sig
      .caption(MethodCaptionStrategy::ArgumentsOnly(
        ArgumentCaptionStrategy::TypeAndName(TypeCaptionStrategy::Short,),
      ))
      .unwrap(),
    "double_arg1"
  );
  assert_eq!(
    sig
      .caption(MethodCaptionStrategy::ArgumentsOnly(
        ArgumentCaptionStrategy::TypeAndName(TypeCaptionStrategy::Full,),
      ))
      .unwrap(),
    "double_arg1"
  );

  assert_eq!(
    sig
      .caption(MethodCaptionStrategy::ConstAndArguments(
        ArgumentCaptionStrategy::NameOnly,
      ))
      .unwrap(),
    "const_arg1"
  );
  assert_eq!(
    sig
      .caption(MethodCaptionStrategy::ConstAndArguments(
        ArgumentCaptionStrategy::TypeOnly(TypeCaptionStrategy::Short,),
      ))
      .unwrap(),
    "const_double"
  );
  assert_eq!(
    sig
      .caption(MethodCaptionStrategy::ConstAndArguments(
        ArgumentCaptionStrategy::TypeOnly(TypeCaptionStrategy::Full),
      ))
      .unwrap(),
    "const_double"
  );
  assert_eq!(
    sig
      .caption(MethodCaptionStrategy::ConstAndArguments(
        ArgumentCaptionStrategy::TypeAndName(TypeCaptionStrategy::Short,),
      ))
      .unwrap(),
    "const_double_arg1"
  );
  assert_eq!(
    sig
      .caption(MethodCaptionStrategy::ConstAndArguments(
        ArgumentCaptionStrategy::TypeAndName(TypeCaptionStrategy::Full,),
      ))
      .unwrap(),
    "const_double_arg1"
  );

  assert!(sig.has_const_this());
}

#[test]
fn cpp_ffi_type_void() {
  let t = CppFfiType::void();
  assert!(t.original_type.is_void());
  assert!(t.ffi_type.is_void());
  assert_eq!(t.conversion, CppTypeConversionToFfi::NoChange);
}

#[test]
fn c_base_name_free_func() {
  let mut method = empty_regular_method();
  method.name = "func1".to_string();
  let include_file = "QRect".to_string();
  assert_eq!(
    c_base_name(
      &method,
      &ReturnValueAllocationPlace::NotApplicable,
      &include_file,
    ).unwrap(),
    "QRect_G_func1"
  );
  assert_eq!(
    c_base_name(&method, &ReturnValueAllocationPlace::Stack, &include_file).unwrap(),
    "QRect_G_func1_to_output"
  );
  assert_eq!(
    c_base_name(&method, &ReturnValueAllocationPlace::Heap, &include_file).unwrap(),
    "QRect_G_func1_as_ptr"
  );
}

#[test]
fn c_base_name_free_func_in_namespace() {
  let mut method = empty_regular_method();
  method.name = "ns::func1".to_string();
  let include_file = "QRect".to_string();
  assert_eq!(
    c_base_name(
      &method,
      &ReturnValueAllocationPlace::NotApplicable,
      &include_file,
    ).unwrap(),
    "QRect_G_ns_func1"
  );
  assert_eq!(
    c_base_name(&method, &ReturnValueAllocationPlace::Stack, &include_file).unwrap(),
    "QRect_G_ns_func1_to_output"
  );
  assert_eq!(
    c_base_name(&method, &ReturnValueAllocationPlace::Heap, &include_file).unwrap(),
    "QRect_G_ns_func1_as_ptr"
  );
}

#[test]
fn c_base_name_class_method() {
  let mut method = empty_regular_method();
  method.name = "func1".to_string();
  method.member = Some(empty_membership("MyClass"));
  let include_file = "QRect".to_string();
  assert_eq!(
    c_base_name(
      &method,
      &ReturnValueAllocationPlace::NotApplicable,
      &include_file,
    ).unwrap(),
    "MyClass_func1"
  );
  assert_eq!(
    c_base_name(&method, &ReturnValueAllocationPlace::Stack, &include_file).unwrap(),
    "MyClass_func1_to_output"
  );
  assert_eq!(
    c_base_name(&method, &ReturnValueAllocationPlace::Heap, &include_file).unwrap(),
    "MyClass_func1_as_ptr"
  );
}

#[test]
fn c_base_name_class_method_in_namespace() {
  let mut method = empty_regular_method();
  method.name = "func1".to_string();
  method.member = Some(empty_membership("ns1::MyClass"));
  let include_file = "QRect".to_string();
  assert_eq!(
    c_base_name(
      &method,
      &ReturnValueAllocationPlace::NotApplicable,
      &include_file,
    ).unwrap(),
    "ns1_MyClass_func1"
  );
  assert_eq!(
    c_base_name(&method, &ReturnValueAllocationPlace::Stack, &include_file).unwrap(),
    "ns1_MyClass_func1_to_output"
  );
  assert_eq!(
    c_base_name(&method, &ReturnValueAllocationPlace::Heap, &include_file).unwrap(),
    "ns1_MyClass_func1_as_ptr"
  );
}

#[test]
fn c_base_name_constructor() {
  let mut method = empty_regular_method();
  method.name = "QRect".to_string();
  method.member = Some({
    let mut info = empty_membership("QRect");
    info.kind = CppFunctionKind::Constructor;
    info
  });
  let include_file = "QtCore".to_string();
  assert!(
    c_base_name(
      &method,
      &ReturnValueAllocationPlace::NotApplicable,
      &include_file,
    ).is_err()
  );
  assert_eq!(
    c_base_name(&method, &ReturnValueAllocationPlace::Stack, &include_file).unwrap(),
    "QRect_constructor"
  );
  assert_eq!(
    c_base_name(&method, &ReturnValueAllocationPlace::Heap, &include_file).unwrap(),
    "QRect_new"
  );
}

#[test]
fn c_base_name_destructor() {
  let mut method = empty_regular_method();
  method.name = "QRect".to_string();
  method.member = Some({
    let mut info = empty_membership("QRect");
    info.kind = CppFunctionKind::Destructor;
    info
  });
  let include_file = "QtCore".to_string();
  assert!(
    c_base_name(
      &method,
      &ReturnValueAllocationPlace::NotApplicable,
      &include_file,
    ).is_err()
  );
  assert_eq!(
    c_base_name(&method, &ReturnValueAllocationPlace::Stack, &include_file).unwrap(),
    "QRect_destructor"
  );
  assert_eq!(
    c_base_name(&method, &ReturnValueAllocationPlace::Heap, &include_file).unwrap(),
    "QRect_delete"
  );
}

#[test]
fn c_base_name_class_method_operator() {
  let mut method = empty_regular_method();
  method.name = "operator>".to_string();
  method.member = Some(empty_membership("MyClass"));
  method.operator = Some(CppOperator::GreaterThan);
  let include_file = "QRect".to_string();
  assert_eq!(
    c_base_name(
      &method,
      &ReturnValueAllocationPlace::NotApplicable,
      &include_file,
    ).unwrap(),
    "MyClass_operator_gt"
  );
  assert_eq!(
    c_base_name(&method, &ReturnValueAllocationPlace::Stack, &include_file).unwrap(),
    "MyClass_operator_gt_to_output"
  );
  assert_eq!(
    c_base_name(&method, &ReturnValueAllocationPlace::Heap, &include_file).unwrap(),
    "MyClass_operator_gt_as_ptr"
  );
}

#[test]
fn c_base_name_free_func_operator() {
  let mut method = empty_regular_method();
  method.name = "operator>".to_string();
  method.operator = Some(CppOperator::GreaterThan);
  let include_file = "QRect".to_string();
  assert_eq!(
    c_base_name(
      &method,
      &ReturnValueAllocationPlace::NotApplicable,
      &include_file,
    ).unwrap(),
    "QRect_G_operator_gt"
  );
  assert_eq!(
    c_base_name(&method, &ReturnValueAllocationPlace::Stack, &include_file).unwrap(),
    "QRect_G_operator_gt_to_output"
  );
  assert_eq!(
    c_base_name(&method, &ReturnValueAllocationPlace::Heap, &include_file).unwrap(),
    "QRect_G_operator_gt_as_ptr"
  );
}

#[test]
fn c_base_name_conversion_operator() {
  let mut method = empty_regular_method();
  method.name = "operator const QPoint&".to_string();
  method.member = Some(empty_membership("MyClass"));
  method.operator = Some(CppOperator::Conversion(CppType {
    is_const: true,
    is_const2: false,
    base: CppTypeBase::Class(CppTypeClassBase {
      name: "QPoint".to_string(),
      template_arguments: None,
    }),
    indirection: CppTypeIndirection::Ref,
  }));
  let include_file = "QRect".to_string();
  assert_eq!(
    c_base_name(
      &method,
      &ReturnValueAllocationPlace::NotApplicable,
      &include_file,
    ).unwrap(),
    "MyClass_convert_to_const_QPoint_ref"
  );
  assert_eq!(
    c_base_name(&method, &ReturnValueAllocationPlace::Stack, &include_file).unwrap(),
    "MyClass_convert_to_const_QPoint_ref_to_output"
  );
  assert_eq!(
    c_base_name(&method, &ReturnValueAllocationPlace::Heap, &include_file).unwrap(),
    "MyClass_convert_to_const_QPoint_ref_as_ptr"
  );
}
