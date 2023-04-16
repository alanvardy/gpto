#!/bin/sh
cd ../gpto-bin/
git pull
cd ../gpto/
makepkg --printsrcinfo > ../gpto-bin/.SRCINFO
mv PKGBUILD ../gpto-bin/
rm *.tar.gz
cd ../gpto-bin/
git add .
git commit -m "new version"
git push aur
cd ../gpto