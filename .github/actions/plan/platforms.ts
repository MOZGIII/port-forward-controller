export type RunnerOS =
  | "ubuntu-22.04"
  | "windows-latest"
  | "macos-14"
  | "macos-14-large";

export type Platform = {
  name: string;
  os: RunnerOS;
  buildEnvScript: string;
  essential: boolean;
  env: Record<string, string>;
  cacheKey: string;
  artifactMarker: string | null;
  isBroken: boolean;
};

export type Platforms = Record<string, Platform>;

// An utility to apply common build script paths.
const buildEnvScriptPath = (script: string) =>
  `.github/scripts/build_env/${script}`;

// All the platforms that we support, and their respective settings.
export const all = {
  ubuntu2204: {
    name: "Ubuntu 22.04",
    os: "ubuntu-22.04",
    buildEnvScript: buildEnvScriptPath("ubuntu.sh"),
    essential: true,
    env: {},
    cacheKey: "ubuntu2204-amd64",
    artifactMarker: "ubuntu2204",
    isBroken: false,
  },
  windows: {
    name: "Windows",
    os: "windows-latest",
    buildEnvScript: buildEnvScriptPath("windows.sh"),
    essential: false,
    env: {},
    cacheKey: "windows-amd64",
    artifactMarker: null,
    isBroken: true,
  },
  macos14amd64: {
    name: "macOS 14 (amd64)",
    os: "macos-14-large",
    buildEnvScript: buildEnvScriptPath("macos.sh"),
    essential: false,
    env: {},
    cacheKey: "macos-14-amd64",
    artifactMarker: null,
    isBroken: true,
  },
  macos14aarch64: {
    name: "macOS 14 (aarch64)",
    os: "macos-14",
    buildEnvScript: buildEnvScriptPath("macos.sh"),
    essential: false,
    env: {},
    cacheKey: "macos-14-aarch64",
    artifactMarker: null,
    isBroken: false,
  },
} satisfies Platforms;

// A platform for running things that are platform-independent.
export const core = all.ubuntu2204 satisfies Platform;
