# Contributing

Contributions are highly appreciated.

## Setting up dev environment

1. Have rustup (and therefore cargo) installed.
2. Fork the repo
3. Clone the fork
4. Do stuff!

## Specific files and directories and what they do or have

The `.prettierrc` file is for configuring prettier, which can be used to format JSON, YAML, and markdown. There are a lot of those files in this repo, so this is quite helpful. If you have VSCode installed, you can add the prettier extension and then use that extension to format files.

The `tests/files` directory is for storing Github responses, so tests do not go over the rate limit, require a real token to run, or require an internet connection.

`src/constants.rs` has fake tokens for auth.

`src/error.rs` is where the library's errors are.

`src/client/*.rs` are the files relating to the client.

`src/utils.rs` has runtime utils.

`src/test_utils.rs` has test utils.

## Contributing a fix or feature

1. Please file an issue first.
2. After some discussion has occured, feel free to file a PR.
    1. Make a branch
    2. Make changes
    3. Commit
    4. Fill out the PR template
    5. Get feedback and refine
    6. Merge

### Commit messages

Follow the [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) specification. Read the spec for more detail.

Read the [AngularJS commit message guidelines](https://github.com/angular/angular/blob/22b96b9/CONTRIBUTING.md#-commit-message-guidelines) for more.

#### Types

-   fix
-   feat
-   chore
-   ci
-   test
-   docs
-   build
-   refactor
-   perf
