
// Third Party
// @TODO-v3-beta: remove
#[cfg(feature = "yaml")]
use yaml_rust::Yaml;

// Internal
use {App, ArgMatches};

/// The abstract representation of a command line subcommand.
///
/// This struct describes all the valid options of the subcommand for the program. Subcommands are
/// essentially "sub-[`App`]s" and contain all the same possibilities (such as their own
/// [arguments], subcommands, and settings).
///
/// # Examples
///
/// ```rust
/// # use clap::{App, Arg, SubCommand};
/// App::new("myprog")
///     .subcommand(
///         App::new("config")
///             .about("Used for configuration")
///             .arg(Arg::new("config_file")
///                 .help("The configuration file to use")
///                 .index(1)))
/// # ;
/// ```
/// [`App`]: ./struct.App.html
/// [arguments]: ./struct.Arg.html
#[derive(Debug, Clone)]
pub struct SubCommand<'a> {
    #[doc(hidden)]
    pub name: String,
    #[doc(hidden)]
    pub matches: ArgMatches<'a>,
}

impl<'a> SubCommand<'a> {
    /// Creates a new instance of a subcommand requiring a name. The name will be displayed
    /// to the user when they print version or help and usage information.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, SubCommand};
    /// App::new("myprog")
    ///     .subcommand(
    ///         App::new("config"))
    /// # ;
    /// ```
    pub fn new<'b>(name: &str) -> App<'a, 'b> { App::new(name) }

    // ------- DEPRECATIONS ---------------

    /// Deprecated
    #[cfg(feature = "yaml")]
    #[deprecated(since = "2.24.1", note = "Use SubCommand::from or serde instead")]
    pub fn from_yaml(yaml: &Yaml) -> App { App::from(yaml) }

    /// Deprecated
    #[deprecated(since = "2.24.1", note = "Use SubCommand::new instead")]
    pub fn with_name<'b>(name: &str) -> App<'a, 'b> { App::new(name) }
}

#[cfg(feature = "yaml")]
impl<'a, 'b, 'z> From<&'z Yaml> for SubCommand<'a> {
    /// Creates a new instance of a subcommand from a YAML (.yml) document
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # #[macro_use]
    /// # extern crate clap;
    /// # use clap::Subcommand;
    /// # fn main() {
    /// let sc_yaml = load_yaml!("test_subcommand.yml");
    /// let sc = SubCommand::from_yaml(sc_yaml);
    /// # }
    /// ```
    fn from(yaml: &'z Yaml) -> App<'a, 'b> { App::from(yaml) }
}
