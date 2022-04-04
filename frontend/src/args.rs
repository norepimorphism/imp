use std::fmt;

#[derive(Default)]
pub struct Args {
    pub script_filepath: Option<String>,
    pub config_filepath: Option<String>,
    pub should_print_vers: bool,
}

impl Args {
    pub fn get() -> Result<Self, Error> {
        let mut settings = Vec::new();

        let mut args = std::env::args()
            // Skip the executable path.
            .skip(1)
            .peekable();

        while args.peek().is_some() {
            settings.push(Setting::parse(&mut args)?);
        }

        let mut args = Self::default();
        for setting in settings {
            match setting.name.as_str() {
                "i" | "in" => {
                    args.script_filepath = Some(setting.value);
                }
                "c" | "config" => {
                    args.config_filepath = Some(setting.value);
                }
                "V" | "version" => {
                    args.should_print_vers = true;
                }
                _ => {
                    return Err(Error::UnknownSetting { name: setting.name });
                }
            }
        }

        Ok(args)
    }
}

struct Setting {
    name: String,
    value: String,
}

impl Setting {
    fn parse(args: &mut impl Iterator<Item = String>) -> Result<Setting, Error> {
        let first = args.next().ok_or(Error::ExpectedSetting)?;
        let (_, name) = first.split_once('-').ok_or(Error::ExpectedSettingName)?;
        let value = args.next().ok_or(Error::ExpectedSettingValue)?;

        Ok(Setting {
            name: name.to_string(),
            value,
        })
    }
}

#[derive(Debug)]
pub enum Error {
    ExpectedSetting,
    ExpectedSettingName,
    ExpectedSettingValue,
    UnknownSetting { name: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ExpectedSetting => "expected setting".to_string(),
                Self::ExpectedSettingName => "expected setting name".to_string(),
                Self::ExpectedSettingValue => "expected setting value".to_string(),
                Self::UnknownSetting { name } => format!("unknown setting '{}'", name),
            }
        )
    }
}
