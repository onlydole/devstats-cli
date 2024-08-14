# DevStats CLI

DevStats CLI is a command-line interface tool for querying contribution statistics from the CNCF's DevStats. It allows users to easily retrieve and view contribution data for specific projects, time ranges, and metrics.

## Features

- Query DevStats API for contribution data
- Customize queries by project, time range, and metric
- Display results in a user-friendly, colored output
- Verbose logging option for troubleshooting

## Installation

### Prerequisites

- Rust programming language (latest stable version)
- Cargo package manager

### Steps

1. Clone the repository:

   ```sh
   git clone https://github.com/yourusername/devstats-cli.git
   cd devstats-cli
   ```

2. Build the project:

   ```sh
   cargo build --release
   ```

3. The binary will be available in `target/release/devstats-cli`

## Usage

Basic usage:

```sh
devstats-cli -u <username>
```

Full usage with all options:

```sh
devstats-cli -u <username> -p <project> -r <range> -m <metric> [-v]
```

Options:

- `-u, --username <USERNAME>`: GitHub username to query (required)
- `-p, --project <PROJECT>`: Project to query (default: "All CNCF")
- `-r, --range <RANGE>`: Time range for the query (default: "Last quarter")
- `-m, --metric <METRIC>`: Metric to query (default: "Contributions")
- `-v, --verbose`: Enable verbose logging

Example:

```sh
devstats-cli -u onlydole -p Kubernetes -r "Last year" -m "Commits"
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [CNCF DevStats platform](https://devstats.cncf.io) for providing the [API](https://github.com/cncf/devstatscode#api)
