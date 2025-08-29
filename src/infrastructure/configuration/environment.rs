/// The possible runtime environment for the application.
pub enum Environment {
    /// Local development environment.
    Local,

    /// Staging environment for testing.
    Staging,

    /// Live environment for deployed instances.
    Production,
}

impl Environment {
    /// Converts the environment to a string representation.
    ///
    /// # Returns
    ///
    /// A static string identifier for the environment.
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Staging => "staging",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    /// Attempts to convert a string into an Environment value.
    ///
    /// # Arguments
    ///
    /// * `s` - String representing the environment.
    ///
    /// # Returns
    ///
    /// * `Ok(Environment)` if the string matches a known environment.
    /// * `Err(String)` with an error message if no match is found.
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "staging" => Ok(Self::Staging),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local`, `staging`, or `production`",
                other
            )),
        }
    }
}
