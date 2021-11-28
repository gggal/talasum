use mockall::automock;
use std::sync::RwLock;

#[derive(Debug, Deserialize)]
struct ConfigFields {
    vertical_randomness_coef: u32,
    horizontal_randomness_coef: u32,
}

#[automock]
pub trait Configurable {
    fn get_vertical_randomness_coef(&self) -> u32;
    fn get_horizontal_randomness_coef(&self) -> u32;
    fn set_horizontal_randomness_coef(&mut self, value: u32);
    fn set_vertical_randomness_coef(&mut self, value: u32);

    fn is_valid_value(value: u32) -> bool {
        value > 0 && value <= 100
    }
}

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
