use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
extern crate proc_macro;

#[proc_macro_derive(SQL)]
pub fn sql_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!()
    };

    let create_contents = fields.into_iter().enumerate().map(|(idx, f)| {
        let ident = &f.ident;
        let ty = &f.ty;

        let ty_ident = grab_field_type_ident(ty).unwrap();

        let sql_type = match ty_ident.to_string().as_str() {
            "String" => "text",
            _ => panic!("Not supported type: {}", ty_ident.to_string()),
        };

        // The last field of a CREATE TABLE query does not have a comma
        if idx == fields.len() - 1 {
            quote! {
                str.push_str(&format!("\t{} {}\n", stringify!(#ident), #sql_type));
            }
        } else {
            quote! {
                str.push_str(&format!("\t{} {},\n", stringify!(#ident), #sql_type));
            }
        }
    });

    let output = quote! {
        impl #name {
            fn create_table() -> String {
                let mut str = String::new();
                str.push_str(&format!("CREATE TABLE {} (\n", stringify!(#name)));
                #(#create_contents)*
                str.push_str(");");
                str
            }
        }
    };

    output.into()
}

fn grab_field_type_ident(ty: &syn::Type) -> Option<syn::Ident> {
    if let syn::Type::Path(ref p) = ty {
        if p.path.segments.len() == 1 {
            return Some(p.path.segments[0].ident.clone());
        }
    }
    None
}
