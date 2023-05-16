#!/bin/bash

cd ../gpto-bin/ || exit
git pull
cd ../gpto/ || exit
makepkg --printsrcinfo > ../gpto-bin/.SRCINFO
mv PKGBUILD ../gpto-bin/
rm *.tar.gz
cd ../gpto-bin/ || exit
git add .
git commit -m "new version"
git push aur
cd ../gpto || exit