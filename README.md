# Actionoscope

Actionoscope is a CLI tool to run steps from a GitHub Actions workflow locally.

## Motivation

### what is it for

Avoid the tedious doom loop of pushing endless commits, waiting ages for github runners to pick your workflow, just to find out that there was yet another typo on the CI/CD workflow.
`actionoscope` is meant to make it dead simple to run locally individual steps (one or multiple) of your GitHub Actions workflow, so you can keep your mental sanity. Nothing more, nothing less.

### what is it _not_ for

`actionoscope` is not meant to:

- work with any other CI/CD tool other than GitHub Actions (forget it Jenkins).
- replace the actual CI/CD pipeline. 

## Features

- Run individual steps from a GitHub Actions workflow.
- Run all steps from a specified step.
- Validate workflow files.

## Installation

To install Actionoscope, you have two options:

a) grab one of the pre-built binaries from the releases page;
b) build it yourself.

To build it yourself, you need to have Rust installed. If you don't have Rust installed, you can install it using [rustup](https://rustup.rs/).
```shell
# clone the repo
git clone git@github.com:diogoaurelio/actionoscope.git

cd actionoscope
# build the project & copy the project to your local bin so you can run it from anywhere as > actionoscope
cargo clean && cargo build --release && cp target/release/actionoscope ~/.local/bin/

# fun
actionoscope --help
```

## Usage

### Running a Single Step
To run a single step from a workflow file:
```shell
actionoscope --workflow-file <path_to_workflow_file> --job <job_name> --step <step_name>
```
Or with short notation:
```shell
actionoscope -w <path_to_workflow_file> -j <job_name> -s <step_name>
```

### Running All Steps Since a Specified Step
To run all steps from a specified step:
```shell
actionoscope --workflow-file <path_to_workflow_file> --job <job_name> --from-step <step_name>
```

Or with short notation:
```shell
actionoscope -w <path_to_workflow_file> -j <job_name> -f <starting_step_name>
```

### Examples
#### Example Workflow File
Here is an example of a GitHub Actions workflow file
```yaml
name: CI
on:
  push:
    branches:
      - main
  pull_request: {}
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        id: first
        uses: actions/checkout@v3
      - name: Run tests
        id: second
        run: cargo test
```
To run the Run tests step from the example workflow file:
```shell
actionoscope --workflow-file example_workflow.yml --job build --step second

# or using just short notation
actionoscope -w example_workflow.yml -j build -s second
```

## Development
### Running Tests
To run the tests for the project:
```shell
RUST_BACKTRACE=1 cargo test --all-features

# OR, sprinkling a bit of inception:
actionoscope -w on.pr.yaml -j build -s test

```

## License
This project is licensed under the MIT License. See the LICENSE file for details.
