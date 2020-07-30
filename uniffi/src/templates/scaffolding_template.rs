// This file was autogenerated by some hot garbage in the `uniffi` crate.
// Trust me, you don't want to mess with it!

{% include "RustBuffer.rs" %}

// We add support for freeing strings, some crates won't need this, but it seems safe
// enough to include anyways since strings are such a common use case.
ffi_support::define_string_destructor!({{ ci.ffi_string_free().name() }});

// For each enum declared in the IDL, we assume the caller as provided a corresponding
// rust `enum`. We provide the traits for sending it across the FFI, which will fail to
// compile if the provided struct has a different shape to the one declared in the IDL.
//
// The enum will be sent over the FFI as a u32, with values assigned according to the
// order of items *as declared in the IDL file*. This might be different to the order
// of items as declared in the rust code, but no harm will come from it.
{% for e in ci.iter_enum_definitions() %}
{% include "EnumTemplate.rs" %}
{% endfor %}

// For each record declared in the IDL, we assume the caller has provided a corresponding
// rust `struct` with the declared fields. We provide the traits for sending it across the FFI.
// If the caller's struct does not match the shape and types declared in the IDL then the rust
// compiler will complain with a type error.
{% for rec in ci.iter_record_definitions() %}
{% include "RecordTemplate.rs" %}
{% endfor %}

// For each top-level function declared in the IDL, we assume the caller has provided a corresponding
// rust function of the same name. We provide a `pub extern "C"` wrapper that does type conversions to
// send data across the FFI, which will fail to compile if the provided function does not match what's
// specified in the IDL.
{%- for func in ci.iter_function_definitions() %}
{% include "TopLevelFunctionTemplate.rs" %}
{% endfor -%}

// For each Object definition, we assume the caller has provided an appropriately-shaped `struct`
// with an `impl` for each method on the object. We create a `ConcurrentHandleMap` for safely handing
// out references to these structs to foreign language code, and we provide a `pub extern "C"` function
// corresponding to each method.
//
// If the caller's implementation of the struct does not match with the methods or types specified
// in the IDL, then the rust compiler will complain with a (hopefully at least somewhat helpful!)
// error message when processing this generated code.
{% for obj in ci.iter_object_definitions() %}
{% include "ObjectTemplate.rs" %}
{% endfor %}

// Finally, we embed a serialiation of the ComponentInterface in the resulting object file,
// making it a self-contained bundle from which bindings can be generated for other languages.
// Putting it in a custom section like this is a little trick from wasm-bingen that I quite liked.
{% let ci_data = ci.to_bincode() %}
#[no_mangle]
#[cfg_attr(any(target_os="macos", target_os="ios"), link_section = "DATA,.uniffi_idl")]
#[cfg_attr(not(any(target_os="macos", target_os="ios")), link_section = ".uniffi_idl")]
pub static UNIFFI_INTERFACE_DEFINITION: [u8;{{ ci_data.len() }}] = [{% for c in ci_data.as_slice() %}{{ c }},{% endfor %}];

{%- import "macros.rs" as rs -%}