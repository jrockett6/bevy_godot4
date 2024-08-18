use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn bevy_app(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let name = &input_fn.sig.ident;
    let expanded = quote! {
        struct BevyExtensionLibrary;

        #[gdextension]
        unsafe impl ExtensionLibrary for BevyExtensionLibrary {
            fn on_level_init(level: bevy_godot4::godot::prelude::InitLevel) {
                if level == bevy_godot4::godot::prelude::InitLevel::Editor {
                    bevy_godot4::godot::private::class_macros::registry::class::auto_register_classes(level);

                    let mut app_builder_func = bevy_godot4::APP_BUILDER_FN.lock().unwrap();
                    if app_builder_func.is_none() {
                        *app_builder_func = Some(Box::new(#name));
                    }
                }
            }

        }

        #input_fn

    };

    expanded.into()
}
