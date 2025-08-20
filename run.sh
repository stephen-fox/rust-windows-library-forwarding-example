#!/bin/sh

set -eux

for dir in targetlib imposterlib app
do
  (
    # The "app" project needs the LIB environment
    # variable to be set to the directory containing
    # the ".lib" file. Maybe there is a way to do
    # this with a rust build script?
    [ "${dir}" == "app" ] && export LIB="${PWD}/targetlib/target/debug"

    cd "${dir}" && cargo clean && cargo build
  )
done

cp targetlib/target/debug/targetlib.dll app/target/debug/targetlib_orig.dll

cp imposterlib/target/debug/targetlib.dll app/target/debug/

./app/target/debug/app.exe
