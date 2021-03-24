use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[proc_macro_derive(Inspectable, attributes(inspectable))]
pub fn inspectable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    match &input.data {
        syn::Data::Struct(data) => expand_struct(&input, data).into(),
        _=> unimplemented!(),
    }
}

fn expand_struct(derive_input: &syn::DeriveInput, data: &syn::DataStruct) -> TokenStream {
    let name = &derive_input.ident;
    let id = name;

    let fields = data.fields.iter().enumerate().map(|(i, field)| {
        let ty = &field.ty;

        let field_label = field_label(field, i);
        let accessor = field_accessor(field, i);

        let (builtin_attributes, custom_attributes): (Vec<_>, Vec<_>) = inspectable_attributes(&field.attrs)
            .partition(InspectableAttribute::is_builtin);

        let mut custom_label = None;
        let mut custom_widget = None;
        for builtin_attribute in builtin_attributes {
            match builtin_attribute {
                InspectableAttribute::Assignment(ident, expr) if ident == "label" => custom_label = Some(expr),
                InspectableAttribute::Assignment(ident, expr) if ident == "widget" => custom_widget = Some(expr),
                InspectableAttribute::Tag(name) | InspectableAttribute::Assignment(name, _) => panic!("unknown attributes '{}'", name),
            }
        }

        let field_label  = match custom_label {
            Some(label) => label.to_token_stream(),
            None => quote! { #field_label },
        };

        let widget = match custom_widget {
            Some(widget) => quote!{
                let row = HBox::new().build(state, panel, |builder| builder);
                let label = Label::new(#field_label).build(state, row, |builder| builder);
                #widget::default().build(state, row, |builder| builder.set_flex_grow(1.0));
            },
            None => quote!{<#ty as tuix_core::Inspectable>::widget(&self.#accessor, state, panel, #field_label);},
        };

        let ui = quote! {
            #widget
            //if let Some(widget) = custom_widget {
            //    widget.to_token_stream()::new().build(state, panel, |builder| builder.set_flex_grow(1.0));
            //} else {
                //<#ty as tuix_core::Inspectable>::widget(&self.#accessor, state, panel, #field_label);
            //}
            //let row = HBox::new().build(state, panel, |builder| builder);
            //let label = Label::new(#field_label).build(state, row, |builder| builder);
        };

        quote! {
            #ui
        }

    });

    quote! {
        impl tuix_core::Inspectable for #name {
           
            fn widget(&self, state: &mut tuix_core::State, parent: tuix_core::Entity, name: &str) -> tuix_core::Entity {
                use tuix_core::widgets::*;

                let panel = Panel::new(stringify!(#id)).build(state, parent, |builder| builder);

                #(#fields)*

                panel

                // let grid = egui::Grid::new(stringify!(#id));
                // grid.show(ui, |ui| {
                //     #(#fields)*
                // });
            }
        }
    }
}

fn field_accessor(field: &syn::Field, i: usize) -> TokenStream {
    match &field.ident {
        Some(name) => name.to_token_stream(),
        None => syn::Index::from(i).to_token_stream(),
    }
}

fn field_label(field: &syn::Field, i: usize) -> String {
    match &field.ident {
        Some(name) => name.to_string(),
        None => i.to_string(),
    }
}


enum InspectableAttribute {
    Assignment(syn::Ident, syn::Expr),
    Tag(syn::Ident),
}
impl InspectableAttribute {
    pub fn ident(&self) -> &syn::Ident {
        match self {
            InspectableAttribute::Assignment(ident, _) => ident,
            InspectableAttribute::Tag(ident) => ident,
        }
    }

    pub fn as_expr(&self) -> TokenStream {
        match self {
            InspectableAttribute::Assignment(_, expr) => quote! { #expr },
            InspectableAttribute::Tag(_) => quote! { true },
        }
    }

    pub fn is_builtin(&self) -> bool {
        let ident = self.ident();
        ident == "label" || ident == "widget"
    }
}

fn parse_inspectable_attributes(
    input: syn::parse::ParseStream,
) -> syn::Result<impl Iterator<Item = InspectableAttribute>> {
    let parse_attribute = |input: syn::parse::ParseStream| {
        let ident: syn::Ident = input.parse()?;
        if input.peek(syn::Token![=]) {
            let _eq_token: syn::Token![=] = input.parse()?;
            let expr: syn::Expr = input.parse()?;
            Ok(InspectableAttribute::Assignment(ident, expr))
        } else if input.is_empty() {
            Ok(InspectableAttribute::Tag(ident))
        } else {
            panic!("could not parse attribute {}", ident);
        }
    };

    input
        .parse_terminated::<_, syn::Token![,]>(parse_attribute)
        .map(IntoIterator::into_iter)
}

/// extracts [(min, 8), (field, vec2(1.0, 1.0))] from `#[inspectable(min = 8, field = vec2(1.0, 1.0))]`,
fn inspectable_attributes(
    attrs: &[syn::Attribute],
) -> impl Iterator<Item = InspectableAttribute> + '_ {
    attrs
        .iter()
        .filter(|attr| attr.path.get_ident().map_or(false, |p| p == "inspectable"))
        .flat_map(|attr| attr.parse_args_with(parse_inspectable_attributes).unwrap())
}