use proc_macro::TokenStream;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    AttrStyle, Attribute, Expr, Ident, ItemMod, Lifetime, Macro, MacroDelimiter, Meta, Path,
    PathSegment, Token, Type, TypePath, TypeReference, Visibility,
};

#[proc_macro_attribute]
pub fn bevy_wgsl(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast: ItemMod = syn::parse(input).unwrap();
    let ast_span = ast.span();
    let mut wgsl_rs_path: Punctuated<PathSegment, Token![::]> = Punctuated::new();
    wgsl_rs_path.push(PathSegment {
        ident: Ident::new("wgsl_rs", ast_span),
        arguments: Default::default(),
    });
    wgsl_rs_path.push(PathSegment {
        ident: Ident::new("wgsl", ast_span),
        arguments: Default::default(),
    });
    if let Some(contents) = ast.content.as_mut() {
        let mut module_path_path: Punctuated<PathSegment, Token![::]> = Punctuated::new();
        module_path_path.push(PathSegment {
            ident: Ident::new("module_path", ast_span),
            arguments: Default::default(),
        });
        let mut str_path: Punctuated<PathSegment, Token![::]> = Punctuated::new();
        str_path.push(PathSegment {
            ident: Ident::new("str", ast_span),
            arguments: Default::default(),
        });
        let module_path: syn::ItemConst = syn::ItemConst {
            attrs: vec![],
            vis: Visibility::Public(Default::default()),
            const_token: Default::default(),
            ident: Ident::new("MODULE_PATH", ast_span),
            generics: Default::default(),
            colon_token: Default::default(),
            ty: Box::new(Type::Reference(TypeReference {
                and_token: Default::default(),
                lifetime: Some(Lifetime {
                    apostrophe: ast_span,
                    ident: Ident::new("static", ast_span),
                }),
                mutability: None,
                elem: Box::new(Type::Path(TypePath {
                    qself: None,
                    path: Path {
                        leading_colon: None,
                        segments: str_path,
                    },
                })),
            })),
            eq_token: Default::default(),
            expr: Box::new(Expr::Macro(syn::ExprMacro {
                attrs: vec![],
                mac: Macro {
                    path: Path {
                        leading_colon: None,
                        segments: module_path_path,
                    },
                    bang_token: Default::default(),
                    delimiter: MacroDelimiter::Paren(Default::default()),
                    tokens: Default::default(),
                },
            })),
            semi_token: Default::default(),
        };
        contents.1.push(syn::Item::Const(module_path));
    }

    ast.attrs.push(Attribute {
        pound_token: Default::default(),
        style: AttrStyle::Outer,
        bracket_token: Default::default(),
        meta: Meta::Path(Path {
            leading_colon: None,
            segments: wgsl_rs_path,
        }),
    });
    ast.to_token_stream().into()
}
