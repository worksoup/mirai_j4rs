use mj_base::MIRAI_PREFIX;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{DeriveInput, LitStr};

fn add_prefix(input: TokenStream) -> LitStr {
    let type_name: &syn::LitStr = &syn::parse(input).expect("类型名称请用字符串表示！");
    let type_name = type_name.value();
    LitStr::new(
        format!("{}{}", MIRAI_PREFIX, type_name).as_str(),
        Span::mixed_site(),
    )
}
/// ### `mj_all`
///
/// 同时应用 [`GetInstanceDerive`], [`AsInstanceDerive`], [`FromInstanceDerive`] 和 [`java_type`](macro@java_type).
///
/// 接受一个字符串字面值参数传递给 `java_type` 属性。
#[proc_macro_attribute]
pub fn mj_all(type_name: TokenStream, input: TokenStream) -> TokenStream {
    let ast: &DeriveInput = &syn::parse(input).unwrap();
    let type_name = add_prefix(type_name);
    let gen = quote! {
        #[jbuchong::java_all(#type_name)]
        #ast
    };
    gen.into()
}
/// ### `MiraiEventDerive`
///
/// 为结构体和枚举类型实现 `MiraiEventTrait`.
///
/// 对结构体或枚举等没有特殊要求。`MiraiEventTrait` 特型会有部分要求，请参看 `mj_internal` 代码。
#[proc_macro_derive(MiraiEventDerive)]
pub fn mirai_event_derive(input: TokenStream) -> TokenStream {
    let ast: &DeriveInput = &syn::parse(input).unwrap();
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics crate::event::MiraiEventTrait for #name #ty_generics #where_clause {
        }
    };
    gen.into()
}

/// ### `mj_event`
///
/// 根据结构体名称应用 [`mj_all`](macro@mj_all) 和 [`MiraiEventDerive`]. 类似于此：
///
/// ```rust
/// use mj_helper_macro::mj_event;
/// #[mj_event]
/// pub struct FriendAddEvent {
///     // Fields.
/// }
///
/// // 相当于下述代码：
/// // 这里的前缀是固定的。
/// #[mj_helper_macro::mj_all("net.mamoe.mirai.event.events.FriendAddEvent")]
/// pub struct FriendAddEvent {
///     // Fields.
/// }
/// ```
///
/// 也可以接受一个字符串字面值参数传递给 `mj_all` 属性，避免生成默认值。
#[proc_macro_attribute]
pub fn mj_event(mj_type: TokenStream, input: TokenStream) -> TokenStream {
    let ast: &DeriveInput = &syn::parse(input).unwrap();
    let type_name = if mj_type.is_empty() {
        let name = &ast.ident;
        LitStr::new(format!("event.events.{name}").as_str(), Span::mixed_site())
    } else {
        syn::parse(mj_type).expect("类型名称请用字符串表示！")
    };
    let gen = quote! {
        #[derive(jbuchong::AsInstanceDerive, jbuchong::TryFromInstanceDerive, jbuchong::GetInstanceDerive, mj_helper_macro::MiraiEventDerive)]
        #[jbuchong::java_type(#type_name)]
        #ast
    };
    gen.into()
}

/// ### `mj_event_without_default_traits`
///
/// 与 [`mj_event`](macro@mj_event) 类似，只是没有应用 [`MiraiEventDerive`].
#[proc_macro_attribute]
pub fn mj_event_without_default_traits(mj_type: TokenStream, input: TokenStream) -> TokenStream {
    let ast: &DeriveInput = &syn::parse(input).unwrap();
    let type_name = if mj_type.is_empty() {
        let name = &ast.ident;
        LitStr::new(format!("event.events.{name}").as_str(), Span::mixed_site())
    } else {
        syn::parse(mj_type).expect("类型名称请用字符串表示！")
    };
    let gen = quote! {
        #[derive(jbuchong::AsInstanceDerive, jbuchong::FromInstanceDerive, jbuchong::GetInstanceDerive)]
        #[jbuchong::java_type(#type_name)]
        #ast
    };
    gen.into()
}
