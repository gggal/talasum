use mockall::automock;
use std::sync::RwLock;

/// Internal representation of the global configuration
/// used for deserialization from file
#[derive(Debug, Deserialize)]
struct ConfigFields {
    vertical_randomness_coef: u32,
    horizontal_randomness_coef: u32,
}

/// Defines rules for interaction with the
/// global configuration
#[automock]
pub trait Configurable {
    /// Returns the vertical randomness coefficient (also mentioned as v-coef
    /// across documentation).
    ///
    /// Vertical randomness determines how extreme the fuzzed result sequence for
    /// individual tokens is. The higher this coefficient is, the more frequently
    /// edge cases occur in the output of the fuzzer. And, respectively, the
    /// lower this value is, the more thorough the fuzzer is in generating new values.
    /// It is recommended that this coefficient is set to a higher value when the
    /// user doesn't have a lot of tries and needs to exhaust as many edge cases as
    /// fast as possible. On the contrary, if the user can afford a longer fuzzing
    /// process, this coefficient should be set to a lower value in order to ensure
    /// covering as many cases as possible.
    ///
    /// To illustrate the need for this coefficient, let's say we have only 10 tries
    /// to generate fuzz values for a JSON string. We want for the value to be null
    /// for (at least) one of these tries, as that is a common and important edge
    /// case for any JSON type. This amounts to 1/10th of all fuzz values being null.
    /// However, if we have 10_000 tries instead, we surely don't want for 1/10th,
    /// or one thousand, of all fuzz values to be null as this would be a waste of
    /// tries.
    ///
    /// Supported values are integers from 1 to 100 (incl), 1 being the min possible
    /// value and 100 being the max possible value.
    fn get_vertical_randomness_coef(&self) -> u32;

    /// Returns the horizontal randomness coefficient (also mentioned as h-coef
    /// across documentation).
    ///
    /// Horizontal randomness determines how extreme the changes to the user
    /// provided input are during mutation. This coefficient is only relevant for
    /// mutation of nested tokens, like a JSON array of object. A high h-coef value
    /// means more severe changes to user's input, thus max h-coef makes
    /// the mutator behave as a generator. A lower h-coef value means more subtle
    /// changes during mutation (but at least one token will be mutated at all times).
    ///
    /// Supported values are integers from 1 to 100 (incl), 1 being the min possible
    /// value and 100 being the max possible value.
    fn get_horizontal_randomness_coef(&self) -> u32;

    /// Saved for future use for updating configuration at runtime.
    fn set_horizontal_randomness_coef(&mut self, value: u32);

    //. Saved for future use for updating configuration at runtime.
    fn set_vertical_randomness_coef(&mut self, value: u32);

    /// Checks whether the coefficients follow the appropriate format
    fn is_valid_value(value: u32) -> bool {
        value > 0 && value <= 100
    }
}

/// An entrypoint to the global configuration -
/// lll interaction with the crate configuration should
/// happen through this struct
pub struct Config {
    inner: RwLock<ConfigFields>,
}

#[automock]
impl Config {
    fn new() -> Self {
        let mut fields = config::Config::default();
        fields
            .merge(config::File::with_name(Self::config_file_name()))
            .expect("Coudln't load config file");
        fields
            .merge(config::Environment::with_prefix(Self::get_env_vars_prefix()))
            .expect("Couldn't load config from env variables");

        let inner = fields
            .try_into::<ConfigFields>()
            .expect("Couldn't deserialize config");

        Config {
            inner: RwLock::new(inner),
        }
    }

    fn config_file_name() -> &'static str {
        println!("debug");
        "Config.toml"
    }

    fn get_env_vars_prefix() -> &'static str {
        "magi"
    }
}

impl Configurable for Config {
    fn get_horizontal_randomness_coef(&self) -> u32 {
        let value = self
            .inner
            .read()
            .expect("Config is unattainable!")
            .horizontal_randomness_coef;
        if Self::is_valid_value(value) {
            value
        } else {
            panic!("Value must be from 1 to 100");
        }
    }

    fn get_vertical_randomness_coef(&self) -> u32 {
        let value = self
            .inner
            .read()
            .expect("Config is unattainable!")
            .vertical_randomness_coef;
        if Self::is_valid_value(value) {
            value
        } else {
            panic!("Value must be from 1 to 100");
        }
    }

    fn set_horizontal_randomness_coef(&mut self, value: u32) {
        if !Self::is_valid_value(value) {
            panic!("Value must be from 1 to 100");
        }
        self.inner
            .write()
            .expect("Config is unattainable!")
            .horizontal_randomness_coef = value;
    }

    fn set_vertical_randomness_coef(&mut self, value: u32) {
        if !Self::is_valid_value(value) {
            panic!("Value must be from 1 to 100");
        }
        self.inner
            .write()
            .expect("Config is unattainable!")
            .vertical_randomness_coef = value;
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

#[cfg(test)]
mod tests {
    use super::{Config, Configurable};
    // use std::env;

    #[test]
    fn env_variables_as_config_values() {
        // default values are acquired from local file
        let config = Config::new();
        assert_eq!(config.get_horizontal_randomness_coef(), 100);
        assert_eq!(config.get_vertical_randomness_coef(), 100);

        // TODO isolate configuration tests from the rest as the
        // following are causing test failures in other modules

        // // env vars override default config values
        // env::set_var("MAGI_HORIZONTAL_RANDOMNESS_COEF", "51");
        // env::set_var("MAGI_VERTICAL_RANDOMNESS_COEF", "49");

        // let config1 = Config::new();
        // assert_eq!(config1.get_horizontal_randomness_coef(), 51);
        // assert_eq!(config1.get_vertical_randomness_coef(), 49);

        // // lowercase env vars override default config values
        // env::set_var("magi_horizontal_randomness_coef", "52");
        // env::set_var("magi_vertical_randomness_coef", "48");

        // let config = Config::new();
        // assert_eq!(config.get_horizontal_randomness_coef(), 52);
        // assert_eq!(config.get_vertical_randomness_coef(), 48);
    }

    #[test]
    fn change_config_at_runtime() {
        let mut config = Config::new();
        config.set_horizontal_randomness_coef(53);
        config.set_vertical_randomness_coef(47);

        assert_eq!(config.get_horizontal_randomness_coef(), 53);
        assert_eq!(config.get_vertical_randomness_coef(), 47);
    }

    #[test]
    #[should_panic]
    fn h_randomness_below_lower_limit_causes_panic() {
        let mut config = Config::new();
        config.set_horizontal_randomness_coef(0);
    }

    #[test]
    #[should_panic]
    fn v_randomness_below_lower_limit_causes_panic() {
        let mut config = Config::new();
        config.set_vertical_randomness_coef(0);
    }

    #[test]
    #[should_panic]
    fn h_randomness_over_upper_limit_causes_panic() {
        let mut config = Config::new();
        config.set_horizontal_randomness_coef(101);
    }

    #[test]
    #[should_panic]
    fn v_randomness_over_upper_limit_causes_panic() {
        let mut config = Config::new();
        config.set_vertical_randomness_coef(101);
    }

    #[test]
    fn max_h_randomness_does_not_cause_panic() {
        let mut config = Config::new();
        config.set_horizontal_randomness_coef(100);
        assert_eq!(config.get_horizontal_randomness_coef(), 100);
    }

    #[test]
    fn max_v_randomness_does_not_cause_panic() {
        let mut config = Config::new();
        config.set_vertical_randomness_coef(100);
        assert_eq!(config.get_vertical_randomness_coef(), 100);
    }

    #[test]
    fn min_h_randomness_does_not_cause_panic() {
        let mut config = Config::new();
        config.set_horizontal_randomness_coef(1);
        assert_eq!(config.get_horizontal_randomness_coef(), 1);
    }

    #[test]
    fn min_v_randomness_does_not_cause_panic() {
        let mut config = Config::new();
        config.set_vertical_randomness_coef(1);
        assert_eq!(config.get_vertical_randomness_coef(), 1);
    }
}
