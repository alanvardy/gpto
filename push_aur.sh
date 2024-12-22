#!/bin/bash

echo "=== CHECKING THAT gpto-bin FOLDER EXITS ===" &&
cd ../gpto-bin/ || exit &&
echo "=== PULLING LATEST AUR ===" &&
git pull &&
echo "=== CHECKING THAT gpto FOLDER EXITS ===" &&
cd ../gpto/ || exit &&
echo "=== MOVING PKGBUILD ===" &&
mv target/cargo-aur/PKGBUILD ./PKGBUILD
echo "=== RUNNING MAKEPKG ===" &&
makepkg --printsrcinfo > ../gpto-bin/.SRCINFO &&
mv PKGBUILD ../gpto-bin/ &&
echo "=== DELETING TAR.GZ ===" &&
rm target/cargo-aur/*.tar.gz &&
cd ../gpto-bin/ || exit &&
echo "=== PUSHING TO AUR ===" &&
git add . &&
git commit -m "v$VERSION" &&
git push aur &&
cd ../gpto || exit &&
echo "=== SUCCESS ===" 
