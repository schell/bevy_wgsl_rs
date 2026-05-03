use proc_macro::TokenStream;
use quote::ToTokens;
use syn::ItemMod;
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn bevy_wgsl(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast: ItemMod = syn::parse(input).unwrap();
    let ast_span = ast.span();
    if let Some(contents) = ast.content.as_mut() {
        let const_tokens = quote::quote_spanned! { ast_span =>
            #[wgsl_ignore]
            pub const MODULE_PATH: &str = module_path!();
        };
        let module_path_const: syn::ItemConst = syn::parse2(const_tokens).unwrap();
        contents.1.push(syn::Item::Const(module_path_const));
    }

    let attr_tokens = quote::quote_spanned! { ast_span =>
        #[wgsl_rs::wgsl]
    };
    let parsed_attrs =
        syn::parse::Parser::parse2(syn::Attribute::parse_outer, attr_tokens).unwrap();
    ast.attrs.extend(parsed_attrs);
    ast.to_token_stream().into()
}
