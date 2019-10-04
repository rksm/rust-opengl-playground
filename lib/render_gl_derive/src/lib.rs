extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// This generates an implementation of shader inputs like
///
/// ```
/// impl Vertex {
///     fn vertex_attrib_pointers(gl: &gl::Gl) {
///         let stride = 6 * std::mem::size_of::<f32>();
///         let location = 0;
///         let offset = 0;
///         unsafe {
///             data::f32_f32_f32::vertex_attrib_pointer(gl, location, stride, offset);
///         }
///         let location = 1;
///         let offset = offset + std::mem::size_of::<data::f32_f32_f32>();
///         unsafe {
///             data::f32_f32_f32::vertex_attrib_pointer(gl, location, stride, offset);
///         }
///     }
/// }
/// ```

#[proc_macro_derive(VertexAttribPointers, attributes(location))]
pub fn vertex_attrib_pointers_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    TokenStream::from(generate_impl(&ast))
}

fn generate_struct_field_vertex_attrib_pointer_call(f: &syn::Field) -> quote::__rt::TokenStream {
    let field_name = match f.ident {
        Some(ref i) => format!("{}", i),
        None => String::from(""),
    };

    let location_val: usize = f
        .attrs
        .iter()
        .find(|attr| {
            attr.path
                .segments
                .first()
                .map(|seg| "location" == seg.ident.to_string())
                .unwrap_or(false)
        })
        .and_then(|attr| attr.parse_meta().ok())
        .and_then(|meta| {
            if let syn::Meta::NameValue(syn::MetaNameValue {
                lit: syn::Lit::Int(n),
                ..
            }) = meta
            {
                n.base10_parse::<usize>().ok()
            } else {
                None
            }
        })
        .unwrap_or_else(|| panic!("Field {} has no #[location = ?]", field_name));

    let field_type = &f.ty;

    let result = quote!(
      let location = #location_val;
      unsafe {
        #field_type::vertex_attrib_pointer(gl, location, stride, offset);
      }
      let offset = offset + std::mem::size_of::<#field_type>();
    );
    result.into()
}

fn generate_vertex_attrib_pointer_calls(body: &syn::Data) -> Vec<quote::__rt::TokenStream> {
    match body {
        syn::Data::Enum(_) => panic!("VertexAttribPointers cannot be implemented for enums"),
        syn::Data::Union(_) => panic!("VertexAttribPointers cannot be implemented for unions"),
        syn::Data::Struct(data) => data
            .fields
            .iter()
            .map(generate_struct_field_vertex_attrib_pointer_call)
            .collect(),
    }
}

fn generate_impl(ast: &DeriveInput) -> quote::__rt::TokenStream {
    let ident = &ast.ident;
    let generics = &ast.generics;
    let where_clause = &ast.generics.where_clause;
    let fields = generate_vertex_attrib_pointer_calls(&ast.data);
    quote! {
      impl #ident #generics #where_clause {
          fn vertex_attrib_pointers(gl: &gl::Gl) {
              let stride = std::mem::size_of::<Self>();
              let offset = 0;
              #(#fields)*
          }
      }
    }
}
