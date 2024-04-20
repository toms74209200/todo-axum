# todo-axum

TODO web app in Rust/axum and OpenAPI Generator

## Environments

- OpenAPI Generator 7.4.0
  - Node.js
  - Java
- Rust 1.76.0
  - axum 0.7.4
- Python/pytest(for Web API testing)

You can set up the environments by using VSCode DevContainer.

## Usage

### Install OpenAPI Generator

```bash
$ npm install -g @openapitools/openapi-generator-cli
```

### Generate Rust/axum code

```bash
$ openapi-generator-cli generate -i reference/spec.yaml -g rust-axum -o ./openapi_gen
```

### Launch Web API server

```bash
todo$ cargo run
   Compiling todo v0.1.0 (/workspaces/todo-axum/todo)
    Finished dev [unoptimized + debuginfo] target(s) in 1.64s
     Running `target/debug/todo`
```

### Test Web API

Install dependencies for testing.

```bash
test$ pip install -r requirements.txt
```

Run tests.

```bash
test$ pytest
```

## License

[MIT License](LICENSE)

## Author

[toms74209200](<https://github.com/toms74209200>)
