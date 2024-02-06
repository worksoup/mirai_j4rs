extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn;
use syn::{Data, DeriveInput, Field, Fields, LitStr, Type};
fn impl_get_as(
    ast_data: &Data,
    name: &proc_macro2::Ident,
    struct_impl: proc_macro2::TokenStream,
    fn_name: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    match &ast_data {
        Data::Struct(_) => struct_impl,
        Data::Enum(data_enum) => {
            let variants = &data_enum.variants;
            let tokens = variants.iter().map(|variant| {
                let ident = &variant.ident;
                quote!(
                    #name::#ident(a) => a.#fn_name(),
                )
            });
            quote!(
                match self {
                    #(#tokens)*
                }
            )
        }
        Data::Union(_) => {
            panic!("不支持使用 `union`!")
        }
    }
}
#[proc_macro_derive(GetInstanceDerive)]
pub fn get_instance_derive(input: TokenStream) -> TokenStream {
    let ast: &DeriveInput = &syn::parse(input).unwrap();
    let name = &ast.ident;
    let generics = &ast.generics;
    let r#impl = impl_get_as(
        &ast.data,
        name,
        quote!(j4rs::Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()),
        quote!(get_instance),
    );
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics mj_base::env::GetInstanceTrait for #name #ty_generics #where_clause {
            fn get_instance(&self) -> j4rs::Instance{
                #r#impl
            }
        }
    };
    gen.into()
}
#[proc_macro_derive(AsInstanceDerive)]
pub fn as_instance_derive(input: TokenStream) -> TokenStream {
    let ast: &DeriveInput = &syn::parse(input).unwrap();
    let name = &ast.ident;
    let generics = &ast.generics;
    let r#impl = impl_get_as(&ast.data, name, quote!(&self.instance), quote!(as_instance));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics mj_base::env::AsInstanceTrait for #name #ty_generics #where_clause {
            fn as_instance(&self) -> &j4rs::Instance{
                #r#impl
            }
        }
    };
    gen.into()
}
#[proc_macro_derive(FromInstanceDerive, attributes(fall))]
pub fn from_instance_derive(input: TokenStream) -> TokenStream {
    fn type_is_phantom_data(field: &Field) -> bool {
        if let Type::Path(ref ty) = field.ty {
            if let Some(ty) = ty.path.segments.last() {
                return ty.ident == "PhantomData";
            }
        }
        false
    }
    fn fill_phantom_data_fields(fields: &Fields) -> proc_macro2::TokenStream {
        let mut tokens = proc_macro2::TokenStream::new();
        for field in fields {
            if type_is_phantom_data(&field) {
                let field_name = field.ident.as_ref();
                if let Some(field_name) = field_name {
                    tokens.extend(quote!(#field_name:std::marker::PhantomData::default(),))
                }
            }
        }
        tokens
    }
    let ast: &DeriveInput = &syn::parse(input).unwrap();
    let name = &ast.ident;
    let generics = &ast.generics;
    let impl_tokens = match &ast.data {
        Data::Struct(s) => {
            let tmp = fill_phantom_data_fields(&s.fields);
            quote!(
                Self{
                        instance,
                        #tmp
                    }
            )
        }
        Data::Enum(e) => {
            let variants = &e.variants;
            let mut fall_arm = variants.first();
            let mut impl_tokens = proc_macro2::TokenStream::new();
            for variant in variants {
                let has_this_attr = if let Some(field_attr) = variant.attrs.first() {
                    if let Some(ident) = field_attr.path().get_ident() {
                        ident == "fall"
                    } else {
                        false
                    }
                } else {
                    false
                };
                if has_this_attr {
                    fall_arm = Some(&variant);
                } else {
                    let ty = match &variant.fields {
                        Fields::Unnamed(fields) => {
                            &fields.unnamed.first().expect("无名枚举没有字段！").ty
                        }
                        _ => {
                            panic!("不支持无内含值的枚举以及有名枚举！")
                        }
                    };
                    let ident = &variant.ident;
                    impl_tokens.extend(quote!(
                        if <#ty as mj_base::env::GetClassTypeTrait>::is_this_type(&instance) {
                            #name::#ident(
                                #ty::from_instance(
                                    <#ty as mj_base::env::GetClassTypeTrait>::cast_to_this_type(instance)
                                )
                            )
                        } else
                    ))
                }
            }
            let fall_arm = fall_arm.expect("须有 `fall` 属性！");
            let fall_arm_ty = match &fall_arm.fields {
                Fields::Unnamed(fields) => &fields.unnamed.first().expect("无名枚举没有字段！").ty,
                _ => {
                    panic!("不支持无内含值的枚举以及有名枚举！")
                }
            };
            let fall_arm_ident = &fall_arm.ident;
            impl_tokens.extend(quote!(
                {#name::#fall_arm_ident(<#fall_arm_ty as mj_base::env::FromInstance>::from_instance(instance))}
            ));
            impl_tokens
        }
        Data::Union(_) => panic!("不支持使用 `union`!"),
    };
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics mj_base::env::FromInstance for #name #ty_generics #where_clause {
            fn from_instance(instance: j4rs::Instance) -> Self{
                #impl_tokens
            }
        }
    };
    gen.into()
}

/// 不要求结构体有 `instance: Instance` 字段。
#[proc_macro_attribute]
pub fn java_type(type_name: TokenStream, input: TokenStream) -> TokenStream {
    let ast: &DeriveInput = &syn::parse(input).unwrap();
    let type_name: &syn::LitStr = &syn::parse(type_name).expect("类型名称请用字符串表示！");
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let gen = quote! {
        #ast
        impl #impl_generics mj_base::env::GetClassTypeTrait for #name #ty_generics #where_clause {
            fn get_class_type() -> j4rs::Instance {
                j4rs::Jvm::attach_thread()
                    .unwrap()
                    .invoke_static(
                        "rt.lea.LumiaUtils",
                        "forName",
                        &[j4rs::InvocationArg::try_from(
                            #type_name,
                        )
                        .unwrap()],
                    )
                    .unwrap()
            }
            fn cast_to_this_type(instance: j4rs::Instance) -> j4rs::Instance {
                let jvm = j4rs::Jvm::attach_thread()
                    .unwrap();
                jvm.cast(&instance, #type_name).unwrap()
            }
            fn get_type_name() -> &'static str {
                #type_name
            }
            fn is_this_type(instance: &j4rs::Instance) -> bool {
                mj_base::utils::is_instance_of(&instance, #type_name)
            }
        }
    };
    gen.into()
}
#[proc_macro_attribute]
pub fn mj_all(type_name: TokenStream, input: TokenStream) -> TokenStream {
    let ast: &DeriveInput = &syn::parse(input).unwrap();
    let type_name: LitStr = syn::parse(type_name).expect("类型名称请用字符串表示！");
    let gen = quote! {
        #[derive(mj_macro::AsInstanceDerive, mj_macro::FromInstanceDerive, mj_macro::GetInstanceDerive)]
        #[mj_macro::java_type(#type_name)]
        #ast
    };
    gen.into()
}

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
#[proc_macro_attribute]
pub fn mj_event(mj_type: TokenStream, input: TokenStream) -> TokenStream {
    let ast: &DeriveInput = &syn::parse(input).unwrap();
    let type_name = if mj_type.is_empty() {
        let name = &ast.ident;
        LitStr::new(
            format!("net.mamoe.mirai.event.events.{name}").as_str(),
            Span::mixed_site(),
        )
    } else {
        syn::parse(mj_type).expect("类型名称请用字符串表示！")
    };
    let gen = quote! {
        #[derive(mj_macro::AsInstanceDerive, mj_macro::FromInstanceDerive, mj_macro::GetInstanceDerive, mj_macro::MiraiEventDerive)]
        #[mj_macro::java_type(#type_name)]
        #ast
    };
    gen.into()
}
#[proc_macro_attribute]
pub fn mj_event_without_default_traits(mj_type: TokenStream, input: TokenStream) -> TokenStream {
    let ast: &DeriveInput = &syn::parse(input).unwrap();
    let type_name = if mj_type.is_empty() {
        let name = &ast.ident;
        LitStr::new(
            format!("net.mamoe.mirai.event.events.{name}").as_str(),
            Span::mixed_site(),
        )
    } else {
        syn::parse(mj_type).expect("类型名称请用字符串表示！")
    };
    let gen = quote! {
        #[derive(mj_macro::AsInstanceDerive, mj_macro::FromInstanceDerive, mj_macro::GetInstanceDerive)]
        #[mj_macro::java_type(#type_name)]
        #ast
    };
    gen.into()
}
