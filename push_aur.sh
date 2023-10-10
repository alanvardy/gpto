#!/bin/bash

cd ../gpto-bin/ || exit
git pull
cd ../gpto/ || exit
mv target/cargo-aur/PKGBUILD ./PKGBUILD
makepkg --printsrcinfo > ../gpto-bin/.SRCINFO
mv PKGBUILD ../gpto-bin/
rm target/cargo-aur/*.tar.gz
cd ../gpto-bin/ || exit
git add .
git commit -m "new version"
git push aur
cd ../gpto || exit
