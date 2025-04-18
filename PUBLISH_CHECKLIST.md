# Publish Checklist

## Setup `gpto-bin`

Create `gpto-bin` directory for pushing to AUR

```fish
./setup_aur.sh
```

## Experimental Publish Procedure

1. Update `CHANGELOG.md` with version number
2. Create PR with

```fish
VERSION=0.2.2 ./create_pr.sh
```

3. Wait for it to pass, then merge and pull in latest changes

```fish
gh pr merge -r --admin && gs
```

4. Release it to all the places

```fish
VERSION=0.2.2 ./release.sh
```

# ---------------------------------------

## Setup gpto-bin

```bash
git clone ssh://aur@aur.archlinux.org/gpto-bin.git
cd gpto-bin
git remote add aur ssh://aur@aur.archlinux.org/gpto-bin.git
```

## Publish to Cargo

This checklist is just here for me to reduce the friction of publishing new versions.

Code changes

1. Update dependencies and make sure nothing broke with `./update_test.sh`
2. Change the version in `Cargo.toml` and in this document (do a global find and replace)
3. Update CHANGELOG.md with the version number
4. Update README.md with help text `cargo run -- -h`
5. Add any new examples to README.md
6. Open PR for the version and wait for it to pass
7. Commit and merge PR

8. Build release

```bash
git checkout main
git pull
cargo aur
```

9. [Create a new release](https://github.com/alanvardy/gpto/releases/new)

- Make sure to use the label and title in format `v0.1.6`
- Add binary from gpto directory

10. Publish to Cargo

```bash
cargo publish
```

11. Push to aur with `./push_aur`
