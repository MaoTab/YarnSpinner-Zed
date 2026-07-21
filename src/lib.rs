use std::fs;
use zed_extension_api::{self as zed, LanguageServerId, Result, settings::LspSettings};

struct YarnLanguageServerBinary {
    path: String,
    args: Option<Vec<String>>,
}

struct YarnSpinnerExtension {
    cached_binary_path: Option<String>,
}

impl YarnSpinnerExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<YarnLanguageServerBinary> {
        let binary_settings = LspSettings::for_worktree("yarn-spinner-lsp", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);
        let binary_args = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.arguments.clone());

        if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
            return Ok(YarnLanguageServerBinary {
                path,
                args: binary_args,
            });
        }

        if let Some(path) = worktree.which("YarnLanguageServer") {
            return Ok(YarnLanguageServerBinary {
                path,
                args: binary_args,
            });
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(YarnLanguageServerBinary {
                    path: path.clone(),
                    args: binary_args,
                });
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = zed::latest_github_release(
            "MaoTab/YarnSpinner",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let platform_name = match platform {
            zed::Os::Mac => "osx",
            zed::Os::Linux => "linux",
            zed::Os::Windows => "win",
        };
        let arch_name = match arch {
            zed::Architecture::Aarch64 => "arm64",
            zed::Architecture::X86 => "x86",
            zed::Architecture::X8664 => "x64",
        };

        let asset_name = format!(
            "yarnspinner.languageserver-{}-{}.zip",
            platform_name, arch_name
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("yarn-spinner-lsp-{}", release.version);
        let binary_name = match platform {
            zed::Os::Windows => "YarnLanguageServer.exe",
            _ => "YarnLanguageServer",
        };
        let binary_path = format!("{}/{}", version_dir, binary_name);

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::Zip,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }

            zed::make_file_executable(&binary_path)?;
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(YarnLanguageServerBinary {
            path: binary_path,
            args: binary_args,
        })
    }
}

impl zed::Extension for YarnSpinnerExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let yarn_binary = self.language_server_binary(language_server_id, worktree)?;
        Ok(zed::Command {
            command: yarn_binary.path,
            args: yarn_binary.args.unwrap_or_else(|| vec![]),
            env: Default::default(),
        })
    }
}

zed::register_extension!(YarnSpinnerExtension);
