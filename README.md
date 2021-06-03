# use-github-api-rs

![CI Status](https://img.shields.io/github/workflow/status/PythonCreator27/use-github-api-rs/CI?label=CI)
![Issues open](https://img.shields.io/github/issues/PythonCreator27/use-github-api-rs)

A GitHub API client for rust.

## WARNING!

This crate is 100% unstable. Almost any useful features are missing at this time. As of now, this crate is mostly skeleton. Breaking changes can happen continuously. This crate does follow SemVer, though. But since this library is still at `0.x`, breaking changes happen in _minor_ versions. If you find any bugs, [report them](https://github.com/PythonCreator27/use-github-api-rs/issues). Please don't file endpoint-related feature requests, as there are plenty already on the list. Filing auth related or api usage related requests (add a new crate feature, expose a field, etc.) is perfectly fine.

## Example

```rust
use use_github_api::{GithubClient, schema::users::list};

let client = GithubClient::builder().auth(env!("GITHUB_TOKEN")).build().unwrap();
let data: Vec<list::User> = client.users().list(list::Params {
    per_page: Some(50),
    since: None,
});
println!("{:?}", data);
// Go to https://api.github.com/users and the response you see there will be the response you see here, minus all of the URL fields.
```

## Auth

Authentication is recommended, and is turned on by default. To disable, use `use-gitub-api = { version = "[version]", no-default-features = true }` in `Cargo.toml`. There are a lot of things that you can't do without auth.

### Getting the auth token

To get the auth token, you will need to go to [GitHub settings](https://github.com/settings/tokens) and generate a token there. Then, set an environment variable there and use it to authenticate with the library.

### Crate features

-   #### `auth`
    Explained above.
-   #### `enterprise`
    Enables usage of a GitHub enterprise instance for the API. Enables the `auth` feature.
-   #### `node_ids`
    See https://docs.github.com/en/graphql/guides/using-global-node-ids. This exposes the node ids.

## Goals

-   Easy to use API
-   Support both enterprise and normal GitHub
-   Support preview features (Search commit API, etc.)
-   Change response schema when deemed fit (remove URLs, etc.)
-   Test things

## TODO (sorted by pritority)

`auth` signalizes that auth is required, and `auth?` signalizes that auth will do something, but is optional.

-   [ ] Finish the client
    -   [ ] Maybe support GitHub AE (once out of preview)
-   [ ] Start on creating the API
    -   [ ] Create the `users` module
        -   [ ] Basic operations
            -   [x] List users
            -   [x] Get a single user - **auth?**
            -   [x] Get contextual information (based on the context) on a user - **auth**
            -   [x] Get the current user - **auth**
            -   [ ] Update the current user - **auth**
        -   [ ] User blocks - **auth**
        -   [ ] Emails - **auth**
        -   [ ] Followers - **auth**
        -   [ ] SSH keys - **auth**
        -   [ ] GPG keys - **auth**
        -   [ ] Create response schema for all of the above
    -   [ ] Create the `repo` module
-   [ ] Polish up the errors
-   [ ] Work with the rate limit
