
// this file is generated — do not edit it


/// <reference types="@sveltejs/kit" />

/**
 * Environment variables [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env`. Like [`$env/dynamic/private`](https://kit.svelte.dev/docs/modules#$env-dynamic-private), this module cannot be imported into client-side code. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://kit.svelte.dev/docs/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://kit.svelte.dev/docs/configuration#env) (if configured).
 * 
 * _Unlike_ [`$env/dynamic/private`](https://kit.svelte.dev/docs/modules#$env-dynamic-private), the values exported from this module are statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * ```ts
 * import { API_KEY } from '$env/static/private';
 * ```
 * 
 * Note that all environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * 
 * ```
 * MY_FEATURE_FLAG=""
 * ```
 * 
 * You can override `.env` values from the command line like so:
 * 
 * ```bash
 * MY_FEATURE_FLAG="enabled" npm run dev
 * ```
 */
declare module '$env/static/private' {
	export const LESSOPEN: string;
	export const USER: string;
	export const npm_config_user_agent: string;
	export const XDG_SEAT: string;
	export const npm_package_dependencies__tauri_apps_api: string;
	export const XDG_SESSION_TYPE: string;
	export const npm_package_devDependencies_vite: string;
	export const npm_node_execpath: string;
	export const SHLVL: string;
	export const TAURI_PLATFORM_VERSION: string;
	export const HOME: string;
	export const OLDPWD: string;
	export const npm_package_devDependencies__typescript_eslint_parser: string;
	export const DESKTOP_SESSION: string;
	export const npm_package_devDependencies_eslint_config_prettier: string;
	export const npm_package_devDependencies_eslint_plugin_svelte: string;
	export const TAURI_ARCH: string;
	export const npm_package_devDependencies__sveltejs_adapter_static: string;
	export const GTK_MODULES: string;
	export const HOMEBREW_PREFIX: string;
	export const npm_package_devDependencies_svelte_check: string;
	export const npm_package_scripts_check: string;
	export const npm_package_scripts_tauri: string;
	export const DBUS_SESSION_BUS_ADDRESS: string;
	export const SYSTEMD_EXEC_PID: string;
	export const TAURI_PLATFORM_TYPE: string;
	export const COLORTERM: string;
	export const npm_package_devDependencies_tailwindcss: string;
	export const npm_package_devDependencies_typescript: string;
	export const INFOPATH: string;
	export const npm_package_scripts_dev: string;
	export const npm_package_devDependencies_prettier: string;
	export const GTK_IM_MODULE: string;
	export const LOGNAME: string;
	export const npm_package_type: string;
	export const npm_package_devDependencies__tauri_apps_cli: string;
	export const TAURI_FAMILY: string;
	export const _: string;
	export const npm_package_private: string;
	export const npm_package_scripts_check_watch: string;
	export const npm_package_devDependencies_autoprefixer: string;
	export const XDG_SESSION_CLASS: string;
	export const npm_package_scripts_lint: string;
	export const npm_package_devDependencies__types_cookie: string;
	export const npm_package_devDependencies__typescript_eslint_eslint_plugin: string;
	export const npm_config_registry: string;
	export const TERM: string;
	export const USERNAME: string;
	export const XDG_SESSION_ID: string;
	export const WINDOWPATH: string;
	export const npm_config_node_gyp: string;
	export const PATH: string;
	export const HOMEBREW_CELLAR: string;
	export const NODE: string;
	export const npm_package_name: string;
	export const GNOME_TERMINAL_SCREEN: string;
	export const XDG_RUNTIME_DIR: string;
	export const npm_config_frozen_lockfile: string;
	export const DISPLAY: string;
	export const TAURI_PLATFORM: string;
	export const LANG: string;
	export const MACOSX_DEPLOYMENT_TARGET: string;
	export const XDG_CURRENT_DESKTOP: string;
	export const npm_package_devDependencies_eslint: string;
	export const GNOME_TERMINAL_SERVICE: string;
	export const LS_COLORS: string;
	export const XAUTHORITY: string;
	export const XDG_SESSION_DESKTOP: string;
	export const XMODIFIERS: string;
	export const npm_lifecycle_script: string;
	export const npm_package_devDependencies__sveltejs_kit: string;
	export const NODE_PATH: string;
	export const SHELL: string;
	export const npm_package_version: string;
	export const npm_lifecycle_event: string;
	export const GDMSESSION: string;
	export const QT_ACCESSIBILITY: string;
	export const npm_package_scripts_build: string;
	export const npm_package_devDependencies_svelte: string;
	export const npm_package_devDependencies_tslib: string;
	export const LESSCLOSE: string;
	export const GPG_AGENT_INFO: string;
	export const QT_IM_MODULE: string;
	export const XDG_VTNR: string;
	export const npm_package_scripts_format: string;
	export const PWD: string;
	export const npm_execpath: string;
	export const CLUTTER_IM_MODULE: string;
	export const TAURI_TARGET_TRIPLE: string;
	export const XDG_CONFIG_DIRS: string;
	export const XDG_DATA_DIRS: string;
	export const HOMEBREW_REPOSITORY: string;
	export const PNPM_SCRIPT_SRC_DIR: string;
	export const npm_package_devDependencies_postcss: string;
	export const npm_command: string;
	export const I3SOCK: string;
	export const npm_package_scripts_preview: string;
	export const npm_package_devDependencies_prettier_plugin_svelte: string;
	export const MANPATH: string;
	export const VTE_VERSION: string;
	export const EDITOR: string;
	export const INIT_CWD: string;
	export const NODE_ENV: string;
}

/**
 * Similar to [`$env/static/private`](https://kit.svelte.dev/docs/modules#$env-static-private), except that it only includes environment variables that begin with [`config.kit.env.publicPrefix`](https://kit.svelte.dev/docs/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Values are replaced statically at build time.
 * 
 * ```ts
 * import { PUBLIC_BASE_URL } from '$env/static/public';
 * ```
 */
declare module '$env/static/public' {
	
}

/**
 * This module provides access to runtime environment variables, as defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/master/packages/adapter-node) (or running [`vite preview`](https://kit.svelte.dev/docs/cli)), this is equivalent to `process.env`. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://kit.svelte.dev/docs/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://kit.svelte.dev/docs/configuration#env) (if configured).
 * 
 * This module cannot be imported into client-side code.
 * 
 * ```ts
 * import { env } from '$env/dynamic/private';
 * console.log(env.DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 * 
 * > In `dev`, `$env/dynamic` always includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 */
declare module '$env/dynamic/private' {
	export const env: {
		LESSOPEN: string;
		USER: string;
		npm_config_user_agent: string;
		XDG_SEAT: string;
		npm_package_dependencies__tauri_apps_api: string;
		XDG_SESSION_TYPE: string;
		npm_package_devDependencies_vite: string;
		npm_node_execpath: string;
		SHLVL: string;
		TAURI_PLATFORM_VERSION: string;
		HOME: string;
		OLDPWD: string;
		npm_package_devDependencies__typescript_eslint_parser: string;
		DESKTOP_SESSION: string;
		npm_package_devDependencies_eslint_config_prettier: string;
		npm_package_devDependencies_eslint_plugin_svelte: string;
		TAURI_ARCH: string;
		npm_package_devDependencies__sveltejs_adapter_static: string;
		GTK_MODULES: string;
		HOMEBREW_PREFIX: string;
		npm_package_devDependencies_svelte_check: string;
		npm_package_scripts_check: string;
		npm_package_scripts_tauri: string;
		DBUS_SESSION_BUS_ADDRESS: string;
		SYSTEMD_EXEC_PID: string;
		TAURI_PLATFORM_TYPE: string;
		COLORTERM: string;
		npm_package_devDependencies_tailwindcss: string;
		npm_package_devDependencies_typescript: string;
		INFOPATH: string;
		npm_package_scripts_dev: string;
		npm_package_devDependencies_prettier: string;
		GTK_IM_MODULE: string;
		LOGNAME: string;
		npm_package_type: string;
		npm_package_devDependencies__tauri_apps_cli: string;
		TAURI_FAMILY: string;
		_: string;
		npm_package_private: string;
		npm_package_scripts_check_watch: string;
		npm_package_devDependencies_autoprefixer: string;
		XDG_SESSION_CLASS: string;
		npm_package_scripts_lint: string;
		npm_package_devDependencies__types_cookie: string;
		npm_package_devDependencies__typescript_eslint_eslint_plugin: string;
		npm_config_registry: string;
		TERM: string;
		USERNAME: string;
		XDG_SESSION_ID: string;
		WINDOWPATH: string;
		npm_config_node_gyp: string;
		PATH: string;
		HOMEBREW_CELLAR: string;
		NODE: string;
		npm_package_name: string;
		GNOME_TERMINAL_SCREEN: string;
		XDG_RUNTIME_DIR: string;
		npm_config_frozen_lockfile: string;
		DISPLAY: string;
		TAURI_PLATFORM: string;
		LANG: string;
		MACOSX_DEPLOYMENT_TARGET: string;
		XDG_CURRENT_DESKTOP: string;
		npm_package_devDependencies_eslint: string;
		GNOME_TERMINAL_SERVICE: string;
		LS_COLORS: string;
		XAUTHORITY: string;
		XDG_SESSION_DESKTOP: string;
		XMODIFIERS: string;
		npm_lifecycle_script: string;
		npm_package_devDependencies__sveltejs_kit: string;
		NODE_PATH: string;
		SHELL: string;
		npm_package_version: string;
		npm_lifecycle_event: string;
		GDMSESSION: string;
		QT_ACCESSIBILITY: string;
		npm_package_scripts_build: string;
		npm_package_devDependencies_svelte: string;
		npm_package_devDependencies_tslib: string;
		LESSCLOSE: string;
		GPG_AGENT_INFO: string;
		QT_IM_MODULE: string;
		XDG_VTNR: string;
		npm_package_scripts_format: string;
		PWD: string;
		npm_execpath: string;
		CLUTTER_IM_MODULE: string;
		TAURI_TARGET_TRIPLE: string;
		XDG_CONFIG_DIRS: string;
		XDG_DATA_DIRS: string;
		HOMEBREW_REPOSITORY: string;
		PNPM_SCRIPT_SRC_DIR: string;
		npm_package_devDependencies_postcss: string;
		npm_command: string;
		I3SOCK: string;
		npm_package_scripts_preview: string;
		npm_package_devDependencies_prettier_plugin_svelte: string;
		MANPATH: string;
		VTE_VERSION: string;
		EDITOR: string;
		INIT_CWD: string;
		NODE_ENV: string;
		[key: `PUBLIC_${string}`]: undefined;
		[key: `${string}`]: string | undefined;
	}
}

/**
 * Similar to [`$env/dynamic/private`](https://kit.svelte.dev/docs/modules#$env-dynamic-private), but only includes variables that begin with [`config.kit.env.publicPrefix`](https://kit.svelte.dev/docs/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Note that public dynamic environment variables must all be sent from the server to the client, causing larger network requests — when possible, use `$env/static/public` instead.
 * 
 * ```ts
 * import { env } from '$env/dynamic/public';
 * console.log(env.PUBLIC_DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 */
declare module '$env/dynamic/public' {
	export const env: {
		[key: `PUBLIC_${string}`]: string | undefined;
	}
}
