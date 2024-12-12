// ///
// /// ```rust no_run
// /// #[derive(Into)]
// /// #[into(TargetStruct)]
// /// struct SourceStruct {
// ///     field1: i32,
// ///     #[into_skip]
// ///     field2: String,
// /// }
// /// ```
// #[proc_macro_derive(Into, attributes(into, into_skip))]
// pub fn derive_into(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);

//     // 获取 #[into(B)] 中的 B
//     let target = input
//         .attrs
//         .iter()
//         .find(|attr| attr.path().is_ident("into"))
//         .expect("Missing #[into(Type)] attribute")
//         .parse_args::<syn::Type>()
//         .expect("Failed to parse target type");

//     // 获取源结构体字段
//     let field_conversions = match &input.data {
//         syn::Data::Struct(data) => match &data.fields {
//             syn::Fields::Named(fields) => fields
//                 .named
//                 .iter()
//                 .filter(|field| !is_skip(field))
//                 .map(|field| {
//                     let name = &field.ident;
//                     quote::quote! { #name: value.#name }
//                 })
//                 .collect::<Vec<_>>(),
//             _ => panic!("Only named fields are supported"),
//         },
//         _ => panic!("Only structs are supported"),
//     };

//     let name = &input.ident;
//     let (_impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

//     let gen = quote::quote! {
//         impl From<#name #ty_generics> for #target #where_clause {
//             fn from(value: #name) -> Self {
//                 #target {
//                     #(#field_conversions,)*
//                 }
//             }
//         }
//     };

//     gen.into()
// }
