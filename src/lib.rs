/// Environment variable struct for better env management.
///
/// Currently it's very opinionated and depends on having
/// `std` as it's fields defaults to `String` type, and
/// uses `std::env::var(<key>)`.
///
/// You are also forced to specify a default-value for the
/// `ENV_VARIABLE` because IMHO that's important.
///
/// Also lastly we just capitalize the field name for
/// env_variable name so make sure to set those up correctly.
///
/// Key roadmap goal is,
/// Support custom aliases for env_var key.
///
/// Note we don't support boolean or enum based env
/// variables yet, I hope to shortly but I don't really
/// need that so haven't thought about it much yet!
///
/// Usage
/// ```rust
/// use env_struct::env_struct;
/// env_struct!{
///     #[derive(Debug)]
///     pub struct DummyEnv {
///         pub path_to_something = "/path_to_something".into(),
///         pub config_path = "/folder/config_path.toml".into(),
///     }
/// };
/// ```
#[macro_export]
macro_rules! env_struct {
    (
        $(#[$outer:meta])*
        $vis:vis struct $struct_name:ident {
            $(
                $vis_ident:vis $field:ident = $fieldDef:expr,
            )*
        }
    ) => {
        $(#[$outer])*
        $vis struct $struct_name {
            $(
                $vis_ident $field: String,
            )*
        }
        impl $struct_name {
            pub fn load_from_env() -> Self {
                let mut env = Self::default();
                $(
                    if let Ok(s) = std::env::var(
                        stringify!($field)
                            .chars()
                            .map(|x| char::to_ascii_uppercase(&x))
                            .collect::<String>(),
                    ) {
                        env.$field = s;
                    }
                )*
                env
            }
        }
        impl Default for $struct_name {
            fn default() -> Self {
                Self {
                    $(
                        $field: $fieldDef,
                    )*
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {

    struct EnvTemp {
        flag: &'static str,
        original_content: Option<String>,
    }
    impl EnvTemp {
        fn set_var(flag: &'static str, val: &'static str) -> EnvTemp {
            let env = EnvTemp {
                flag,
                original_content: std::env::var(flag).ok(),
            };
            std::env::set_var(flag, val);
            env
        }
    }
    impl Drop for EnvTemp {
        fn drop(&mut self) {
            // reset_var
            if let Some(og) = &self.original_content {
                std::env::set_var(self.flag, og);
            } else {
                std::env::remove_var(self.flag);
            }
        }
    }

    #[test]
    fn test() {
        let hello_sam = "Hello, Sam!";
        let welp_sam = "Welp, Sam!";
        let temp_env = [
            EnvTemp::set_var("HELLO_WORLD", hello_sam),
            EnvTemp::set_var("WELP_MY_WORLD", welp_sam),
        ];
        env_struct! {
            struct Env {
                hello_world = "hello".into(),
                welp_my_world = "welp".into(),
            }    
        }
        let env = Env::load_from_env();
        assert_eq!(env.hello_world, hello_sam);
        assert_eq!(env.welp_my_world, welp_sam);
        drop(temp_env); // drop would be called without this as well
    }
}
