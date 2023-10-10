#!/bin/bash

cd ~/dev || exit
git clone ssh://aur@aur.archlinux.org/gpto-bin.git
cd gpto-bin || exit
git remote add aur ssh://aur@aur.archlinux.org/gpto-bin.git
cd ../gpto || exit
