
0.2.0:
* docs(readme): add instructions for skipping steps
* refactor(tests/workflow_tests): prefer TDD test structure
* refactor(tests/step_tests): prefer TDD test structure
* refactor(tests/cli_tests): prefer TDD test structure
* feat(cli): add ability to skip a given amount of steps
* refactor(tests/cli_tests): clean up test files
* test(tests): add file utils fns to create dir & rm file
* build(tests): add tear down test context
* refactor(tests/cli_tests): use constant for workflow file name
* refactor(tests/cli_tests): use constant for app name
* refactor(lib): rename stdout disclaimer
* refactor(cli): use directly string interpolation in println and format macros
* refactor(lib): use directly string interpolation in println and format macros


0.1.0:
* docs(cargo): update project info
* docs(readme): update & improve project description
* refactor(cli): rm unnecessary return statement
* fix(.github): enable linter check to fail on errors
* refactor(lib): simplify references to collections::HashMap
* feat(lib): add ability to extract github config automatically
* feat(lib & cli): add ability to inject .env file as a secrets file
* refactor(cli): improve displaying workflows
* feat(cli): add 'run' and 'ls' commands
* refactor(cli): move running jobs into dedicated fn
* build(.github/actions): change rust setup strategy
* feat(lib & cli): relax constraint of having to pass specific job or step
* feat(cli): relax on constraint of having to pass which workflow to run
* feat(lib & cli): add ability to run only until a given step
* build(.github): add ci pipeline
* docs(readme): add repo intro doc
* feat(lib & cli): add first version of library & cli
* docs(license): add repo license
* build(.gitignore): add basics to exclude in git
