# env_struct

[![docs.rs/env-struct](https://img.shields.io/docsrs/env_struct)](https://docs.rs/env-struct)
[![crates.io/crates/env-struct](https://buildstats.info/crate/env-struct)](https://crates.io/crates/env-struct)


This crate is very opinionated env management crate for
building good env management habits and avoiding global variables
if possible, and having sane defaults.

Can still be used with `lazy_init` to load the struct as a
global static. But that isn't baked into this. In future one should
be able to use the new lazy `once_cell` in API in rust_std once
it gets stablized.

A quick example of usage looks like,
```rust
use env_struct::env_struct;

env_struct! {
    #[derive(Debug)] // (Optional) Not needed, just to
    //  show that we keep derive & other macros intact
    pub struct ConfigEnvWithDefault { // vis modifiers work too
        // Will use `CONFIG_PATH`
        pub config_path = "/path/to/config.toml", 
        // Will use `RESULT_PATH`
        pub result_path = "/path/to/result.toml", 
    } 
}

pub fn main() {
    let env_config = ConfigEnvWithDefault::load_from_env();

    if let Some(s) = entry_point(&env_config) {
        eprintln!("Program exited successfullly!");
        post_run_operations(&env_config);
        // we don't care if the post run operations
        // such as cleanup or such fail.
    } else {
        eprintln!("Program exit status invalid!");
        std::process::exit(1);
    }
}
```

Or for config where you want to ensure you have env setup,
```rust
use env_struct::env_struct;

env_struct! {
    #[derive(Debug)] // (Optional) Not needed, just to
    //  show that we keep derive & other macros intact
    pub struct RequiredConfigEnv { // vis modifiers work too
        // Will use `CONFIG_PATH`
        pub config_path, 
        // Will use `RESULT_PATH`
        pub result_path,
    }
}


pub fn main() {
    // Result<RequiredConfigEnv, String>
    // Error: String above will name the var that couldn't be read.
    let env_config = RequiredConfigEnv::try_load_from_env().unwrap();

    if let Some(s) = entry_point(&env_config) {
        eprintln!("Program exited successfullly!");
        post_run_operations(&env_config);
        // we don't care if the post run operations
        // such as cleanup or such fail.
    } else {
        eprintln!("Program exit status invalid!");
        std::process::exit(1);
    }
}
```

## My Opinions

Environment variable structs are a better for env management.

As then you can ensure the nature of sync on top of env on your own,
and don't have to pay the cost of `Atomic`s without needing to use them
in your single threaded application.

Also this works better when your system already has a context provider,
for dependency injection, for instance, in a GUI application.

And, defaults should always exist if only to inform the user of the lack of configuration.

## Roadmap
- Support custom aliases for env_var key.

## Dependencies
None, and the entire project is less than 100 lines of code.

## Contribution
Open to accept PR to fix any bugs or add more quality of life features,
just try to follow the "keep it minimal" philosophy.

I don't want to ship features that aren't directly relevant or would be soon implemented in rust std and are provided by most other crates.

This is supposed to be a very very minimal project.