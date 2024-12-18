use proc_macro::TokenStream;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, Result as SynResult, Token};

// 定义解析 into 属性参数的结构体
struct IntoArgs {
    target_type: syn::Type,
    use_default: bool,
}

impl Parse for IntoArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let target_type = input.parse()?;
        let use_default = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            input.parse::<syn::Ident>()? == "default"
        } else {
            false
        };
        Ok(IntoArgs {
            target_type,
            use_default,
        })
    }
}

/// impl Into<T> for Struct
///
/// ```rust no_run
/// #[derive(Debug, Default)]
/// struct Foo {
///     field1: i32,
///     field3: String,
/// }
///
/// #[into(Foo)]  // no default
/// //#[into(Foo, default)]  // with default field
/// struct Bar {
///     field1: i32,
///     #[into_skip]
///     field2: String,
///     #[into(self.field3.to_string())]
///     field3: i32,
/// }
/// ```
#[proc_macro_attribute]
pub fn into(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as IntoArgs);
    let target = args.target_type;
    let use_default = args.use_default;
    let mut input = parse_macro_input!(input as syn::ItemStruct);

    // 获取源结构体字段, 生成字段转换
    let field_conversions = match &input.fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter()
            .filter(|field| !is_skip(field, "into_skip"))
            .map(|field| {
                let name = &field.ident;
                field
                    .attrs
                    .iter()
                    .find(|attr| attr.path().is_ident("into"))
                    .map(|attr| {
                        let value = attr.parse_args::<syn::Expr>().unwrap();
                        quote::quote!(#name: #value)
                    })
                    .unwrap_or_else(|| quote::quote!(#name: self.#name))
            })
            .collect::<Vec<_>>(),
        _ => panic!("Only named fields are supported"),
    };

    // 去掉 #[into_skip] 属性, 否则会报错
    match &mut input.fields {
        syn::Fields::Named(fields) => {
            fields.named.iter_mut().for_each(|field| {
                field.attrs.retain(|attr| {
                    !attr.path().is_ident("into_skip") && !attr.path().is_ident("into")
                });
            });
        },
        _ => panic!("Only named fields are supported"),
    }

    let name = &input.ident;

    let struct_init = if use_default {
        quote::quote! {
            #target {
                #(#field_conversions,)*
                ..Default::default()
            }
        }
    } else {
        quote::quote! {
            #target {
                #(#field_conversions,)*
            }
        }
    };

    let gen = quote::quote! {
        #input

        impl Into<#target> for #name {
            fn into(self) -> #target {
                #struct_init
            }
        }
    };

    gen.into()
}

fn is_skip(field: &syn::Field, name: &str) -> bool {
    // 跳过含有指定属性的字段
    field.attrs.iter().any(|attr| attr.path().is_ident(name))
}

/// impl From<T> for Struct
///
/// ```rust no_run
/// struct Foo {
///     field1: i32,
///     field2: String,
/// }
///
/// #[from(Foo)]
/// struct Bar {
///     field1: i32,
///     #[from(source.field2.parse::<i32>().unwrap())]
///     field3: i32,
/// }
///
/// ```
#[proc_macro_attribute]
pub fn from(args: TokenStream, input: TokenStream) -> TokenStream {
    // 解析目标类型参数
    let target = parse_macro_input!(args as syn::Type);
    let mut input = parse_macro_input!(input as syn::ItemStruct);

    // 获取源结构体字段, 生成字段转换
    let field_conversions = match &input.fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| {
                let name = &field.ident;
                // 如果字段有 #[from] 属性, 则使用该属性值的表达式作为value
                field
                    .attrs
                    .iter()
                    .find(|attr| attr.path().is_ident("from"))
                    .map(|a| {
                        let value = a.parse_args::<syn::Expr>().unwrap();
                        quote::quote!(#name: #value)
                    })
                    .unwrap_or(quote::quote!(#name: source.#name))
                // quote::quote! { #name: source.#name}
            })
            .collect::<Vec<_>>(),
        _ => panic!("Only named fields are supported"),
    };

    // 去掉 #[from] 属性, 否则会报错
    match &mut input.fields {
        syn::Fields::Named(fields) => {
            fields.named.iter_mut().for_each(|field| {
                field.attrs.retain(|attr| !attr.path().is_ident("from"));
            });
        },
        _ => panic!("Only named fields are supported"),
    }

    let name = &input.ident;
    // let vis = &input.vis;
    // let (_impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let gen = quote::quote! {
        #input

        impl From<#target> for #name {
            fn from(source: #target) -> Self {
                #name {
                    #(#field_conversions,)*
                }
            }
        }
    };

    gen.into()
}
