# o7d CLI

The command line application for o7d, designed to streamline infrastructure management and local development setup.

## Commands

### `setup`

Installs all required dependencies for local o7d development using Homebrew.

Currently installs:

- KIND (Kubernetes in Docker)
- kubectl (Kubernetes command-line tool)
- Helm (Kubernetes package manager)

Usage:

```bash
o7d setup
```

### `init`

Initializes a standardized infrastructure repository structure for managing Kubernetes resources across different environments.

Default directory structure:

```bash
├── apps/             # Application manifests
│   ├── base/           # Base configurations
│   ├── prod/           # Production overrides
│   └── dev/            # Staging overrides
├── infrastructure/   # Infrastructure components
│   ├── base/
│   ├── prod/
│   └── dev/
└── clusters/         # Cluster-specific configurations
    ├── prod/
    └── dev/
```

Usage:

Basic initialization with default environments (base, production, staging):

```bash
o7d init
```

Custom environments:

```bash
o7d init --envs production development staging
# or
o7d init -e production development staging
```

This will create the same directory structure but with your specified environments instead of the defaults. The `base` directory is always created regardless of specified environments.
