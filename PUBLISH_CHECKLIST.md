# Publish Checklist

## Publish to Cargo

This checklist is just here for me to reduce the friction of publishing new versions.

Code changes

1. Run `cargo update` to make sure dependencies are up to date
2. Run `./test.sh && ./manual_test.sh` to make sure that didn't break anything
3. Revert the change to the test `gpto.cfg`
4. Change the version in Cargo.toml and in this document (do a global find and replace)
5. Update CHANGELOG.md with the version number
6. Update README.md with help text `cargo run -- -h`
7. Add any new examples to README.md
8. Open PR for the version and wait for it to pass
9. Commit and merge PR

10. Publish to Cargo

```bash
cargo publish
```
