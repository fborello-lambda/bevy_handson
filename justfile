default: lint build-debug test

build-release:
    cargo build --release
alias br := build-release

build-debug:
    cargo build
alias bd := build-debug

test:
    cargo test
alias t := test

lint:
    cargo clippy
alias l := lint

[positional-arguments]
@git_tag *args='':
    git tag $1 -f
    git push --tags -f
