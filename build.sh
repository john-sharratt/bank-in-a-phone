#!/bin/bash -e
cd crates/web
trunk build --release
cd ../..
git add crates/web/dist
git commit -a -m "Next release"
git push
