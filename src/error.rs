use serde_json;
use serde_yaml;
use toml;

use std::io;

error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links {
    }

    foreign_links {
        Io(io::Error);
        Json(serde_json::Error);
        Yaml(serde_yaml::Error);
        Toml(toml::de::Error);
    }

    errors {
    }
}
