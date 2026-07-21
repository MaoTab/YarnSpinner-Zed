# Bundled Yarn Spinner Runtime

This runtime is derived from the official Yarn Spinner VS Code extension version 3.2.113.

- `language-server/server.js` is bundled with esbuild 0.25.8 for Node.js on Windows; the optional macOS-only `fsevents` package is externalized.
- `compiler-service` contains the assemblies required to run `YarnSpinner.CompilerService.dll` with .NET 9. Debug symbols, XML documentation, apphost, and localized Roslyn resources are omitted.
- See `LICENSE.md` for the upstream license.
