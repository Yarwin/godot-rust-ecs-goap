use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::{DeriveInput, Ident, parse_macro_input};
use syn::spanned::Spanned;


fn bad_format_error<T: ToTokens>(tokens: T) -> proc_macro2::TokenStream {
    syn::Error::new_spanned(tokens, "expected `from_field(\"field_name\")` or `expose_by_resource`").to_compile_error()
}

fn is_resource_field(f: &syn::Field) -> Option<&syn::Attribute> {
    if f.attrs.len() == 0 {return None; }
    for attr in &f.attrs {
        if attr.path.segments.len() == 1 && (attr.path.segments[0].ident == "expose_by_resource" || attr.path.segments[0].ident == "from_field") {
            return Some(attr);
        }
    }
    None
}

enum GodotResourceField<'a> {
    Included(&'a syn::Field),
    CopiedFrom(&'a syn::Field, proc_macro2::Ident),
    Error(proc_macro2::TokenStream)
}

#[proc_macro_derive(ComponentGodotResource, attributes(from_field, expose_by_resource))]
pub fn derive_trait(input: TokenStream) -> TokenStream {
    // eprintln!("{:#?}", input);
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;
    let res_name = format!("{}Resource", name);
    let res_ident = Ident::new(&*res_name, name.span());
    let res_ident_in_map = Ident::new("resource_instance", name.span());
    let name_in_map = Ident::new(&*"component", name.span());
    let fields = if let syn::Data::Struct(syn::DataStruct {fields: syn::Fields::Named(syn::FieldsNamed {ref named, ..}), ..}) = ast.data {
        named
    } else {
        unimplemented!();
    };

    // deal with attributes
    let fields_with_attributes = fields.iter().filter_map(|f| {
        let g = is_resource_field(f)?;
        return match g.parse_meta() {
            Ok(syn::Meta::NameValue(_nv)) => {
                None
            }
            Ok(syn::Meta::List(mut nvs)) => {
                if nvs.path.get_ident().unwrap() != "from_field" {
                    return Some(GodotResourceField::Error(bad_format_error(nvs)));
                }

                if nvs.nested.len() != 1 {
                    return Some(GodotResourceField::Error(bad_format_error(nvs)));
                }
                if let syn::NestedMeta::Lit(syn::Lit::Str(s)) = nvs.nested.pop().unwrap().into_value() {
                    let arg = Ident::new(&s.value(), s.span());
                    return Some(GodotResourceField::CopiedFrom(f, arg));
                }

                None
            }
            Ok(syn::Meta::Path(mut m)) => {
                if m.segments.len() != 1 {
                    return Some(GodotResourceField::Error(bad_format_error(m)));
                }
                let segment = m.segments.pop().unwrap().into_value();
                if segment.ident != "expose_by_resource" {
                    return Some(GodotResourceField::Error(bad_format_error(m)));
                }
                Some(GodotResourceField::Included(f))
            }
            Err(e) => {
                Some(GodotResourceField::Error(e.to_compile_error()))
            }
        }
    });

    let resource_fields = fields_with_attributes.clone().filter_map(|field_enum|
        match field_enum {
            GodotResourceField::Included(f) => {
                let name = &f.ident;
                let new_ty = f.ty.clone();
                if let syn::Type::Path(f_ty) = f.ty.clone() {
                    if f_ty.path.segments[0].ident == "usize" {
                        let new_ident = Ident::new("i32", f.ty.span().clone());
                        return Some(quote! {
                            #[property]
                            #name: #new_ident
                        });
                    }
                };

                Some(quote! {
                    #[property]
                    #name: #new_ty
                })
            }
            _ => None
        }
    );

    let fields_included = fields_with_attributes.clone().filter_map(|field_enum|
        match field_enum {
            GodotResourceField::Included(f) => {
                let field_name = &f.ident;
                let _field_ty = &f.ty;
                if let syn::Type::Path(f_ty) = f.ty.clone() {
                    if f_ty.path.segments[0].ident == "usize" {
                        return Some(quote! {
                            #name_in_map.#field_name = #res_ident_in_map.#field_name as usize
                        })
                    }

                }

                Some(quote! {
            #name_in_map.#field_name = #res_ident_in_map.#field_name.clone()
                })
            }
            _ => None
        }
    );

    let fields_copied = fields_with_attributes.clone().filter_map(|field_enum|
        match field_enum {
            GodotResourceField::CopiedFrom(f, i) => {
                let field_name = &f.ident;
                return Some(
                    quote! {
                        #name_in_map.#field_name = #name_in_map.#i
                    });
            },
            GodotResourceField::Error(e) => Some(e),
            _ => None
        }
    );


    let name_str = format!("{}", name);
    let expanded = quote! {

        impl GodotResourceComponent for #name {
            fn from_resource(resource: Ref<Resource>) -> Self {
                let mut #name_in_map = #name::default();
                let resource = resource.cast_instance::<#res_ident>().expect("failed to cast to native script");
                unsafe { resource.assume_safe() }.map(|#res_ident_in_map: &#res_ident, _| {
                    #(#fields_included;)*
                }).unwrap();
                #(#fields_copied;)*

                #name_in_map
            }
        }

        #[derive(::core::default::Default)]
        #[derive(::gdnative::derive::NativeClass)]
        #[inherit(::gdnative::api::Resource)]
        #[derive(::gdnative::derive::ToVariant, ::gdnative::derive::FromVariant)]
        #[automatically_derived]
        pub struct #res_ident {
            #(#resource_fields,)*
        }

        #[methods]
        impl #res_ident {
            fn new(_owner: &Resource) -> Self {
                #res_ident::default()
            }

            #[export]
            fn get_component_type(&mut self, _owner: &Resource) -> GodotString {
                GodotString::from(#name_str)
            }
        }


    };
    expanded.into()
}

#[proc_macro_derive(GoapAction, attributes(implementation))]
pub fn derive_goap_action(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident: enum_name_ident,
        data,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let (is_valid, perform, clear, get_cost) = match data {
        syn::Data::Enum(my_enum) => (
            gen_description_str_for_enum(&my_enum,
                                         "is_valid",
                                         quote! {current_state},
                                         &enum_name_ident
            ),
            gen_description_str_for_enum(&my_enum,
                                         "perform",
                                         quote! {working_memory, owner, world, global_state, blackboard, global_facts},
                                         &enum_name_ident
            ),
            gen_description_str_for_enum(&my_enum,
                                         "clear",
                                         quote!{blackboard},
                                         &enum_name_ident
            ),
            gen_description_str_for_enum(&my_enum,
                                         "get_cost",
                                         quote!{original_cost, working_memory},
                                         &enum_name_ident
            )
        ),
        _ => panic!("Only enums are supported for now on")
    };

    let expanded = quote! {
        impl GoapAction for #enum_name_ident {
            fn is_valid(&self, current_state: &GoapPlannerWorkingFacts) -> bool {
                match self {
                    #is_valid
                }
            }

            fn get_cost(&self, original_cost: u32, working_memory: &GoapWorkingMemoryFacts) -> u32 {
                match self {
                    #get_cost
                }
            }

            fn perform(&mut self,
                working_memory: &mut GoapWorkingMemoryFacts,
                owner: Entity,
                world: &mut World,
                global_state: &GlobalStateResource,
                blackboard: &mut Instance<GoapBlackboardNode>,
                global_facts: &GoapPlannerWorkingFacts) -> bool {

                match self {
                    #perform
                }
            }

            fn clear(&self, blackboard: &mut Instance<GoapBlackboardNode>) {
                match self {
                    #clear
                }
            }
        }

    };
    expanded.into()
}

#[proc_macro_derive(GoapGoal, attributes(implementation))]
pub fn derive_goap_goal(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident: enum_name_ident,
        data,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let (is_valid, prority) = match data {
        syn::Data::Enum(my_enum) => (
            gen_description_str_for_enum(&my_enum,
                                         "is_valid",
                                         quote! {current_memory, current_facts},
                                         &enum_name_ident
            ),
            gen_description_str_for_enum(&my_enum,
                                         "priority",
                                         quote! {original_priority, current_memory},
                                         &enum_name_ident
            ),
            ),
        _ => panic!("Only enums are supported for now on")
    };

    let expanded = quote! {
        impl GoapGoal for #enum_name_ident {
            fn is_valid(&self, current_memory: &GoapWorkingMemoryFacts, current_facts: &GoapPlannerWorkingFacts) -> bool {
                match self {
                    #is_valid
                }
            }

            fn priority(&self, original_priority: u32, current_memory: &GoapWorkingMemoryFacts) -> u32 {
                match self {
                    #prority
                }
            }
        }
    };
    expanded.into()
}

fn gen_description_str_for_enum(my_enum: &syn::DataEnum, method_name: &str, args: proc_macro2::TokenStream, enum_name_ident: &proc_macro2::Ident) -> proc_macro2::TokenStream {
    my_enum
        .variants
        .iter()
        .filter_map(|it| {
            let field_name = &it.ident;
            if it.attrs.len() == 0 {
                return None;
            }
            let att = &it.attrs[0];
            if let Ok(syn::Meta::NameValue(nv)) = att.parse_meta() {
                if let syn::Lit::Str(s) = nv.lit {
                    let arg: proc_macro2::TokenStream = s.value().parse().unwrap();
                    let method_declaration: proc_macro2::TokenStream = method_name.parse().unwrap();
                    return Some( quote! {
                        #enum_name_ident::#field_name => #arg::#method_declaration(#args),
                    });
                }
            }
            None
        }).collect()
}
