#![feature(let_chains)]

use convert_case::{Case, Casing};
use mj_base::MIRAI_PREFIX;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_macro_input, DeriveInput, FnArg, GenericArgument, ItemFn, LitStr, PatType,
    PathArguments::AngleBracketed, Stmt, Type,
};

/// ### `mj_all`
///
/// 同时应用 [`GetInstanceDerive`], [`AsInstanceDerive`], [`FromInstanceDerive`] 和 [`java_type`](macro@java_type).
///
/// 接受一个字符串字面值参数传递给 `java_type` 属性。
#[proc_macro_attribute]
pub fn mj_all(type_name: TokenStream, input: TokenStream) -> TokenStream {
    fn add_prefix(input: TokenStream) -> LitStr {
        let type_name: &syn::LitStr = &syn::parse(input).expect("类型名称请用字符串表示！");
        let ty = type_name.value();
        LitStr::new(format!("{}{}", MIRAI_PREFIX, ty).as_str(), type_name.span())
    }
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
        impl #impl_generics crate::event::MiraiEventTrait<B> for #name #ty_generics #where_clause {
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
/// #[mj_helper_macro::mj_all("event.events.FriendAddEvent")]
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
        #[derive(mj_helper_macro::MiraiEventDerive)]
        #[mj_helper_macro::mj_all(#type_name)]
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

#[proc_macro_attribute]
pub fn java_fn(java_fn_name: TokenStream, input: TokenStream) -> TokenStream {
    let fn_item = parse_macro_input!(input as ItemFn);
    let vis = &fn_item.vis;
    let fn_name = &fn_item.sig.ident;
    let java_fn_name = if java_fn_name.is_empty() {
        let s = fn_name.to_string().to_case(Case::Camel);
        quote! {
            #s
        }
    } else {
        java_fn_name.into()
    };
    let generics = &fn_item.sig.generics;
    let inputs = &fn_item.sig.inputs;
    let output = &fn_item.sig.output;
    let where_clause = &fn_item.sig.generics.where_clause;
    let block = &fn_item.block;

    // 是否只有注释。
    let mut only_comments = true;
    for stmt in &block.stmts {
        if let Stmt::Expr(syn::Expr::Verbatim(_), ..) = stmt {
            continue;
        } else {
            only_comments = false;
            break;
        }
    }
    // 生成参数处理代码
    let mut modified_inputs = Vec::new();
    let mut arg_conversions = Vec::new();
    let mut args = Vec::new();
    let mut self_arg = None;
    let mut jvm_var = quote! {jvm};
    let mut def_jvm = quote! {
        let #jvm_var = j4rs::Jvm::attach_thread().unwrap();
    };
    fn handle_ty(
        ty: &Type,
        pat_type: &PatType,
        and_token: Option<&syn::Token![&]>,
        arg_conversions: &mut Vec<proc_macro2::TokenStream>,
        modified_inputs: &mut Vec<proc_macro2::TokenStream>,
        args: &mut Vec<proc_macro2::TokenStream>,
        def_jvm: &mut proc_macro2::TokenStream,
        jvm_var: &mut proc_macro2::TokenStream,
    ) {
        let pat = &pat_type.pat;
        let arg = if and_token.is_some() {
            quote! {
                let #pat = jbuchong::ToArgTrait::to_arg(#pat).unwrap();
            }
        } else {
            quote! {
                let #pat = jbuchong::IntoArgTrait::into_arg(#pat).unwrap();
            }
        };
        match ty {
            Type::Path(syn::TypePath { path, .. }) => {
                if let Some(last_segments) = path.segments.last() {
                    match last_segments.ident.to_string().as_str() {
                        "DataWrapper" => {
                            if and_token.is_some() {
                                panic!("不支持引用的 `DataWrapper` 类型！")
                            }
                            if let AngleBracketed(angle_bracketed_args) = &last_segments.arguments
                                && let Some(GenericArgument::Type(inner_type)) =
                                    angle_bracketed_args.args.first()
                            {
                                let arg = quote! {
                                    let #pat = <#ty>::from(#pat);
                                    #arg
                                };
                                arg_conversions.push(arg);
                                modified_inputs.push(quote! {#pat: #inner_type});
                                args.push(quote! {
                                    #pat
                                });
                            } else {
                                arg_conversions.push(arg);
                                modified_inputs.push(quote! {#pat_type});
                                args.push(quote! {#pat})
                            }
                        }
                        "Jvm" => {
                            *def_jvm = quote! {};
                            *jvm_var = quote! {#pat};
                            modified_inputs.push(quote! {#pat_type});
                        }
                        _ => {
                            arg_conversions.push(arg);
                            modified_inputs.push(quote! {#pat_type});
                            args.push(quote! {
                                #pat
                            });
                        }
                    }
                } else {
                    modified_inputs.push(quote! {#pat_type});
                    arg_conversions.push(arg);
                    args.push(quote! {
                        #pat
                    });
                }
            }
            Type::Reference(syn::TypeReference {
                elem: ty,
                and_token,
                ..
            }) => handle_ty(
                ty,
                pat_type,
                Some(and_token),
                arg_conversions,
                modified_inputs,
                args,
                def_jvm,
                jvm_var,
            ),
            _ => {
                panic!("不支持此类型！你可以定义一个内部函数，然后进行包装。")
            }
        }
    }
    inputs.iter().for_each(|arg| match arg {
        FnArg::Receiver(r) => {
            self_arg = Some(r);
            modified_inputs.push(quote! {#r});
        }
        FnArg::Typed(pat_type) => {
            let ty = &pat_type.ty;
            handle_ty(
                ty,
                pat_type,
                None,
                &mut arg_conversions,
                &mut modified_inputs,
                &mut args,
                &mut def_jvm,
                &mut jvm_var,
            )
        }
    });
    // 生成返回值处理代码
    let return_conversion = if let syn::ReturnType::Type(_, ty) = output
        && only_comments
    {
        quote! {
            <#ty as jbuchong::FromInstanceTrait>::from_instance(instance)
        }
    } else {
        quote! {
            #block
        }
    };
    let java_call = if self_arg.is_some() {
        quote! {
            let instance = #jvm_var.invoke(&<Self as jbuchong::GetInstanceTrait>::get_instance(&self).unwrap(), #java_fn_name, infer_type(&[#(#args),*])).unwrap();
        }
    } else {
        quote! {
            let instance = #jvm_var.invoke_static(&<Self as jbuchong::GetClassTypeTrait>::get_type_name(), #java_fn_name, infer_type(&[#(#args),*])).unwrap();
        }
    };
    let gen = quote! {
        #vis fn #fn_name #generics(#(#modified_inputs),*) #output #where_clause {
            fn infer_type(in_: &[InvocationArg]) -> &[InvocationArg] {
                in_
            }
            #def_jvm
            #(#arg_conversions)*
            #java_call
            #return_conversion
        }
    };
    gen.into()
}
/// 抑制错误信息。
#[proc_macro]
pub fn error_msg_suppressor(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let input_str = input_str
        .value()
        .parse::<proc_macro2::TokenStream>()
        .unwrap();
    let gen = quote! {
        #input_str
    };
    gen.into()
}
