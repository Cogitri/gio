use glib::variant::FromVariant;
use glib::{BoolError, IsA, ToVariant, VariantType};
use {Settings, SettingsExt};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GetError {
    WrongType(VariantType, String),
    KeyNotFound(String),
}

impl std::fmt::Display for GetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GetError::WrongType(request_type, key_type) => write!(
                f,
                "GetError: Type mismatch. Requested type '{}', but key is of type '{}'",
                request_type, key_type
            ),
            GetError::KeyNotFound(key) => write!(f, "GetError: Couldn't find key '{}'", key),
        }
    }
}

impl std::error::Error for GetError {
    fn description(&self) -> &str {
        match self {
            GetError::WrongType(_, _) => "GetError: Type mismatch",
            GetError::KeyNotFound(_) => "GetError: Couldn't find key",
        }
    }
}

pub trait SettingsExtManual {
    fn get<U: FromVariant>(&self, key: &str) -> Result<U, GetError>;

    fn set<U: ToVariant>(&self, key: &str, value: &U) -> Result<(), BoolError>;
}

impl<O: IsA<Settings>> SettingsExtManual for O {
    fn get<U: FromVariant>(&self, key: &str) -> Result<U, GetError> {
        if let Some(val) = self.get_value(key) {
            if let Some(ret) = FromVariant::from_variant(&val) {
                Ok(ret)
            } else {
                Err(GetError::WrongType(
                    val.type_().to_owned(),
                    U::static_variant_type().to_str().to_string(),
                ))
            }
        } else {
            Err(GetError::KeyNotFound(key.to_string()))
        }
    }

    fn set<U: ToVariant>(&self, key: &str, value: &U) -> Result<(), BoolError> {
        self.set_value(key, &value.to_variant())
    }
}
