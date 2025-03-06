#[macro_export]
macro_rules! environment_variables {
    ( $( $name:ident: $type:tt = $default_value:expr ),* $(,)? ) => {
        pub mod config {
            use std::{
                sync::LazyLock,
                env,
                any::type_name,
            };

            fn get_variable_type<T>(_: &T) -> &'static str {
                type_name::<T>().split("::").last().unwrap()
            }

            $(
                pub static $name: LazyLock<$type> = LazyLock::new(|| {
                    let name = stringify!($name);

                    if let Ok(value) = env::var(name) {
                        if let Ok(value) = value.parse::<$type>() {
                            value
                        } else {
                            panic!(
                                "Invalid value type for the variable `{name}`! Expected type `{}`, got `{}`.",
                                stringify!($type),
                                get_variable_type(&value)
                            )
                        }
                    } else {
                        log::warn!(
                            "Variable `{}` is missing in the env! Using default value `{}`",
                            name,
                            $default_value
                        );
                        <$type>::from($default_value)
                    }
                });
            )*

            pub fn init() {
                $(
                    LazyLock::force(&$name);
                )*
            }
        }
    };
}
