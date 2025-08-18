#!/bin/sh

set -eux

for dir in targetlib evil app
do
  (
    # The app needs LIB to be set to the directory
    # containing the ".lib" file. Maybe there is
    # a way to do this with a Cargo build script?
    [ "${dir}" == "app" ] && export LIB="${PWD}/targetlib/target/debug"

    cd "${dir}" && cargo clean && cargo build
  )
done

cp targetlib/target/debug/targetlib.dll app/target/debug/targetlib_orig.dll

cp evil/target/debug/targetlib.dll app/target/debug/

( cd app/target/debug && ./app.exe )
