use crate::engine::Engine;

use anyhow::anyhow;
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Default)]
pub struct EngineOptions {
    #[derivative(Default(value = "5"))]
    pub move_overhead: usize,
    #[derivative(Default(value = "1"))]
    pub threads: usize,
    #[derivative(Default(value = "16"))]
    pub hash: usize,
    pub debug: bool,
}

impl Engine {
    pub fn set_option(&mut self, name: String, value: Option<String>) {
        macro_rules! parse_set_field {
            ($field:ident, $min:expr, $max:expr) => {{
                let value = value
                    .ok_or(anyhow!("Missing value for {}", stringify!($field)))
                    .and_then(|v| v.parse().map_err(anyhow::Error::from));

                match value {
                    Ok(value) => {
                        if ($min..=$max).contains(&value) {
                            self.options.$field = value;
                        } else {
                            println!(
                                "Value for {} must be between {} and {}",
                                stringify!($field),
                                $min,
                                $max
                            );
                        }
                    }
                    Err(e) => println!("Couldn't set value for {}: {}", stringify!($field), e),
                }
            }};
        }

        match name.replace(' ', "").to_lowercase().as_str() {
            "moveoverhead" => parse_set_field!(move_overhead, 0, 1000),
            "threads" => parse_set_field!(threads, 1, 1024),
            "hash" => parse_set_field!(hash, 1, 33554432),
            "debug" => self.options.debug = true,
            _ => println!("Unknown option {name}"),
        }
    }
}
