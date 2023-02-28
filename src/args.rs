use clap::Parser;

#[derive(Parser)]
#[command(verbatim_doc_comment)]
///
/// Shell script using natural language
///
/// Uses AI to convert natural descriptions
/// into runnable shell scripts.
///
/// Internet and some setup is required. The
/// required AI language models are currently
/// behind proprietary, remote APIs.
///
/// Setup can be done by file or environment
/// variables:
///
/// ```sh
/// #
/// # Configuration from file
/// #
/// $ shai configure --openai-api-key ...
/// $ shai "Path of the largest file on the system"
///
/// #
/// # Configuration from environment variables
/// #
/// $ OPENAI_API_KEY=... shai "Path of the largest file on the system"
/// ```
///
/// Obtain an API key here: https://platform.openai.com/account/api-keys
///
pub enum Args {
    Command(Command),
    Configure(Configure),
}

#[derive(Parser)]
///
/// Print out the command for the given natural description
///
pub struct Command {
    ///
    /// Natural description for generated command
    ///
    pub description: String,
}

#[derive(Parser)]
#[command(verbatim_doc_comment)]
///
/// Configure shai and save to file
///
pub struct Configure {
    #[arg(long, value_name = "KEY")]
    ///
    /// OpenAI API key
    ///
    /// https://platform.openai.com/account/api-keys
    ///
    pub openai_api_key: String,
}
