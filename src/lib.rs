use zed_extension_api::{
    self as zed, set_language_server_installation_status, Command,
    LanguageServerInstallationStatus, Result,
};

struct Zolidity {}

impl zed::Extension for Zolidity {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {}
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Command> {
        set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::Downloading,
        );
        let command = worktree.which("solidity-analyzer-ls");
        Ok(Command {
            command: command.ok_or("solidity-analyzer-ls not found")?,
            args: vec![],
            env: vec![],
        })
    }
}

zed::register_extension!(Zolidity);
