# Publish Checklist

## Setup gpto-bin

```bash
git clone ssh://aur@aur.archlinux.org/gpto-bin.git
cd gpto-bin
git remote add aur ssh://aur@aur.archlinux.org/gpto-bin.git
```

## Publish to Cargo

This checklist is just here for me to reduce the friction of publishing new versions.

Code changes

1. Update dependencies and make sure nothing broke

```bash
cargo update && \
./test.sh && \
./manual_test.sh
```

2. Revert the change to the test `gpto.cfg`
3. Change the version in `Cargo.toml` and in this document (do a global find and replace)
4. Update CHANGELOG.md with the version number
5. Update README.md with help text `cargo run -- -h`
6. Add any new examples to README.md
7. Open PR for the version and wait for it to pass
8. Commit and merge PR

9. Build release

```bash
git checkout main
git pull
cargo aur
```

10. [Create a new release](https://github.com/alanvardy/gpto/releases/new)

- Make sure to use the label and title in format `v0.1.5`
- Add binary from gpto directory

11. Publish to Cargo

```bash
cargo publish
```

12. Make sure we have the latest AUR git history

```bash
cd ../gpto-bin/
git pull
cd ../gpto/
```

13. Push to AUR

```bash
makepkg --printsrcinfo > ../gpto-bin/.SRCINFO
mv PKGBUILD ../gpto-bin/
rm *.tar.gz
cd ../gpto-bin/
git add .
git commit -m v0.1.5
git push aur
```
