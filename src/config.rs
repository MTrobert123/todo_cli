use std::env;

pub struct Signs {
    pub warning: String,
    pub done: String,
    pub not_done: String,
}

impl Signs {
    pub fn get_signs() -> Signs {
        let fallback_signs = Signs {
            warning: "!!!".to_string(),
            done: "x".to_string(),
            not_done: " ".to_string(),
        };
        let pretty_signs = Signs {
            warning: "⚠️".to_string(),
            done: "✔️".to_string(),
            not_done: "❌".to_string(),
        };
        match env::var("DISABLE_TODO_PRETTY") {
            Ok(value) => {
                if value == "1" {
                    return fallback_signs;
                } else {
                    return pretty_signs;
                }
            }
            Err(_) => {
                return pretty_signs;
            }
        }
    }
}
