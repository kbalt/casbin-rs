initSidebarItems({"attr":[["export_fn","Attribute, when put on a Rust function, turns it into a plugin function."],["export_module","Attribute, when put on a Rust module, turns it into a plugin module."]],"constant":[["OP_CONTAINS","Standard containment testing function."],["OP_EQUALS","Standard equality comparison operator."]],"enum":[["EvalAltResult","Evaluation result."],["FnAccess","A type representing the access mode of a function."],["FnNamespace","A type representing the namespace of a function."],["LexError","Error encountered when tokenizing the script text."],["ParseErrorType","Error encountered when parsing a script."]],"macro":[["combine_with_exported_module","Macro to combine a plugin module into an existing module."],["def_package","Macro that makes it easy to define a package (which is basically a shared [module][Module]) and register functions into it."],["exported_module","Macro to generate a Rhai `Module` from a plugin module defined via [`#[export_module]`][macro@export_module]."],["register_exported_fn","Macro to register a plugin function (defined via [`#[export_fn]`][macro@export_fn]) into an `Engine`."],["reify","Macro to cast an identifier or expression to another type with type checks."],["set_exported_fn","Macro to register a plugin function into a Rhai `Module`."],["set_exported_global_fn","Macro to register a plugin function into a Rhai `Module` and expose it globally."]],"mod":[["packages","Module containing all built-in packages available to Rhai, plus facilities to define custom packages."],["plugin","Module defining macros for developing plugins."],["serde","(serde) Serialization and deserialization support for `serde`. Exported under the `serde` feature only."]],"struct":[["AST","Compiled AST (abstract syntax tree) of a Rhai script."],["Dynamic","Dynamic type containing any value."],["Engine","Rhai main scripting engine."],["EvalContext","Context of a script evaluation process."],["Expression","An expression sub-tree in an [`AST`][crate::AST]."],["FnPtr","A general function pointer, which may carry additional (i.e. curried) argument values to be passed onto a function during a call."],["ImmutableString","The system immutable string type."],["Instant","A measurement of a monotonically nondecreasing clock. Opaque and useful only with [`Duration`]."],["Locked","A reader-writer lock"],["Module","A module which may contain variables, sub-modules, external Rust functions, and/or script-defined functions."],["NativeCallContext","Context of a native Rust function call."],["ParseError","Error when parsing a script."],["Position","A location (line number + character position) in the input script."],["Scope","Type containing information about the current scope. Useful for keeping state between [`Engine`][crate::Engine] evaluation runs."],["Shared","A thread-safe reference-counting pointer. ‘Arc’ stands for ‘Atomically Reference Counted’."],["VarDefInfo","Information on a variable definition."]],"trait":[["FuncArgs","Trait that parses arguments to a function call."],["RegisterNativeFunction","Trait to register custom Rust functions."]],"type":[["Array","Variable-sized array of [`Dynamic`] values."],["Blob","Variable-sized array of [`u8`] values (byte array)."],["INT","The system integer type. It is defined as [`i32`] since the `only_i32` feature is used."],["Map","A dictionary of [`Dynamic`] values with string keys."],["OptimizationLevel","Placeholder for the optimization level."]]});