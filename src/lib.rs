use std::collections::HashMap;

use zed_extension_api::{self as zed, Result, settings::LspSettings};

const EXTENSION_ID: &str = "yarn-spinner";
const LANGUAGE_SERVER_ENTRYPOINT: &str = "runtime\\language-server\\server.js";

struct YarnLanguageServerCommand {
    command: String,
    args: Vec<String>,
    env: Vec<(String, String)>,
}

struct YarnSpinnerExtension;

fn settings_env(env: Option<HashMap<String, String>>) -> Vec<(String, String)> {
    env.unwrap_or_default().into_iter().collect()
}

fn installed_extension_path(local_app_data: &str, relative_path: &str) -> String {
    let local_app_data = local_app_data.trim_end_matches(['\\', '/']);
    format!("{local_app_data}\\Zed\\extensions\\installed\\{EXTENSION_ID}\\{relative_path}")
}

impl YarnSpinnerExtension {
    fn language_server_command(
        &self,
        worktree: &zed::Worktree,
    ) -> Result<YarnLanguageServerCommand> {
        let binary_settings = LspSettings::for_worktree("yarn-spinner-lsp", worktree)
            .ok()
            .and_then(|settings| settings.binary);
        let binary_args = binary_settings
            .as_ref()
            .and_then(|settings| settings.arguments.clone())
            .unwrap_or_default();
        let binary_env = settings_env(
            binary_settings
                .as_ref()
                .and_then(|settings| settings.env.clone()),
        );

        if let Some(path) = binary_settings.and_then(|settings| settings.path) {
            return Ok(YarnLanguageServerCommand {
                command: path,
                args: binary_args,
                env: binary_env,
            });
        }

        // Keep supporting installations of the legacy native language server.
        if let Some(path) = worktree.which("YarnLanguageServer") {
            return Ok(YarnLanguageServerCommand {
                command: path,
                args: binary_args,
                env: binary_env,
            });
        }

        let (platform, _) = zed::current_platform();
        if platform != zed::Os::Windows {
            return Err(
                "The bundled Yarn Spinner language server currently supports Windows only."
                    .to_string(),
            );
        }

        let node_path = worktree.which("node").ok_or_else(|| {
            "Yarn Spinner requires Node.js. Install Node.js and make `node` available in PATH."
                .to_string()
        })?;

        let configured_dotnet_path = binary_env
            .iter()
            .find(|(key, _)| key == "DOTNET_PATH")
            .map(|(_, value)| value.clone());
        let dotnet_path = configured_dotnet_path
            .or_else(|| worktree.which("dotnet"))
            .ok_or_else(|| {
                "Yarn Spinner requires .NET 9. Install it and make `dotnet` available in PATH, or set `DOTNET_PATH` in the language server environment."
                    .to_string()
            })?;

        let shell_env = worktree.shell_env();
        let local_app_data = shell_env
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case("LOCALAPPDATA"))
            .map(|(_, value)| value)
            .ok_or_else(|| {
                "Yarn Spinner could not find LOCALAPPDATA in the shell environment.".to_string()
            })?;

        let server_path = installed_extension_path(local_app_data, LANGUAGE_SERVER_ENTRYPOINT);
        Ok(Self::node_command(
            node_path,
            server_path,
            binary_args,
            binary_env,
            dotnet_path,
        ))
    }

    fn node_command(
        node_path: String,
        server_path: String,
        binary_args: Vec<String>,
        mut binary_env: Vec<(String, String)>,
        dotnet_path: String,
    ) -> YarnLanguageServerCommand {
        let mut args = vec![server_path, "--stdio".to_string()];
        args.extend(binary_args);

        if !binary_env.iter().any(|(key, _)| key == "DOTNET_PATH") {
            binary_env.push(("DOTNET_PATH".to_string(), dotnet_path));
        }

        YarnLanguageServerCommand {
            command: node_path,
            args,
            env: binary_env,
        }
    }
}

impl zed::Extension for YarnSpinnerExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let command = YarnSpinnerExtension::language_server_command(self, worktree)?;
        Ok(zed::Command {
            command: command.command,
            args: command.args,
            env: command.env,
        })
    }
}

zed::register_extension!(YarnSpinnerExtension);

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn builds_the_windows_installed_extension_path() {
        assert_eq!(
            installed_extension_path(
                r"C:\Users\MaoTou\AppData\Local\",
                LANGUAGE_SERVER_ENTRYPOINT
            ),
            r"C:\Users\MaoTou\AppData\Local\Zed\extensions\installed\yarn-spinner\runtime\language-server\server.js"
        );
    }

    #[test]
    fn bundled_runtime_files_are_present() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        assert!(root.join("runtime/language-server/server.js").is_file());
        assert!(
            root.join("runtime/compiler-service/YarnSpinner.CompilerService.dll")
                .is_file()
        );
    }

    #[test]
    fn node_command_uses_stdio_and_exposes_dotnet() {
        let command = YarnSpinnerExtension::node_command(
            "node".to_string(),
            "server.js".to_string(),
            vec!["--trace".to_string()],
            vec![("CUSTOM".to_string(), "value".to_string())],
            "dotnet".to_string(),
        );

        assert_eq!(command.command, "node");
        assert_eq!(command.args, ["server.js", "--stdio", "--trace"]);
        assert!(
            command
                .env
                .contains(&("DOTNET_PATH".to_string(), "dotnet".to_string()))
        );
    }

    #[test]
    fn node_command_preserves_a_configured_dotnet_path() {
        let command = YarnSpinnerExtension::node_command(
            "node".to_string(),
            "server.js".to_string(),
            Vec::new(),
            vec![("DOTNET_PATH".to_string(), "custom-dotnet".to_string())],
            "detected-dotnet".to_string(),
        );

        assert_eq!(
            command
                .env
                .iter()
                .filter(|(key, _)| key == "DOTNET_PATH")
                .collect::<Vec<_>>(),
            [&("DOTNET_PATH".to_string(), "custom-dotnet".to_string())]
        );
    }
}
