extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::{Data, Field, Fields, Type};

#[proc_macro_derive(GetInstanceDerive)]
pub fn get_instance_derive(input: TokenStream) -> TokenStream {
    let ast: &syn::DeriveInput = &syn::parse(input).unwrap();
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics mj_base::env::GetInstanceTrait for #name #ty_generics #where_clause {
            fn get_instance(&self) -> j4rs::Instance{
                j4rs::Jvm::attach_thread().unwrap().clone_instance(&self.instance).unwrap()
            }
        }
    };
    gen.into()
}
#[proc_macro_derive(AsInstanceDerive)]
pub fn as_instance_derive(input: TokenStream) -> TokenStream {
    let ast: &syn::DeriveInput = &syn::parse(input).unwrap();
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics mj_base::env::AsInstanceTrait for #name #ty_generics #where_clause {
            fn as_instance(&self) -> &j4rs::Instance{
                &self.instance
            }
        }
    };
    gen.into()
}
#[proc_macro_derive(FromInstanceDerive)]
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
    let ast: &syn::DeriveInput = &syn::parse(input).unwrap();
    let filled_fields = match &ast.data {
        Data::Struct(s) => fill_phantom_data_fields(&s.fields),
        _ => proc_macro2::TokenStream::new(),
    };
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics mj_base::env::FromInstance for #name #ty_generics #where_clause {
            fn from_instance(instance: j4rs::Instance) -> Self{
                Self{
                    instance,
                    #filled_fields
                }
            }
        }
    };
    gen.into()
}

/// 不要求结构体有 `instance: Instance` 字段。
#[proc_macro_attribute]
pub fn java_type(type_name: TokenStream, input: TokenStream) -> TokenStream {
    let ast: &syn::DeriveInput = &syn::parse(input).unwrap();
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
    let ast: &syn::DeriveInput = &syn::parse(input).unwrap();
    let type_name: &syn::LitStr = &syn::parse(type_name).expect("类型名称请用字符串表示！");
    let gen = quote! {
        #[derive(mj_macro::AsInstanceDerive, mj_macro::FromInstanceDerive, mj_macro::GetInstanceDerive)]
        #[mj_macro::java_type(#type_name)]
        #ast
    };
    gen.into()
}
