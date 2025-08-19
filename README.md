# Windows library forwarding example in Rust

*Note: This code is for educational purposes only.*

This repository provides a full end-to-end example of proxying a Windows
dynamic library using library forwarding in Rust.

Library forwarding can be useful in reverse engineering contexts where
a hacker would like to achieve code execution in a program by replacing
one of its dynamic libraries with a hacker-controlled library. The hacker's
library "proxies" the library by telling the OS to forward function calls
to a renamed copy of the real library. Since the OS defers to the original
library's implementation, the hacker does not need to reimplement functions
or worry about function signature changes from one library version
to another.

Library forwarding relies on the Windows compiler's [module-definition (DEF)][def]
file format. For a full explanation of this technique, please refer to
["dll-hijack-by-proxying"][tothi-repo] by GitHub user [tothi][tothi]. I also
recommend checking out Sam's ["Creating A DLL With Rust" blog posts"][sam-blog].

ps: I use the term "library forwarding" loosely here - I do not believe
there is a colloquial term that describes such functionality. Other operating
systems' compilers may support this as well (refer to the `See also` section
for details). Full disclosure: this is a well known technique. I just wanted
to better understand how it "works" and save everyone from learning the
various Windows-isms and Rust-isms involved in making it work in Rust.

[def]: https://learn.microsoft.com/en-us/cpp/build/exporting-from-a-dll-using-def-files?view=msvc-170
[tothi-repo]: https://github.com/tothi/dll-hijack-by-proxying
[tothi]: https://github.com/tothi
[sam-blog]: https://samrambles.com/guides/window-hacking-with-rust/creating-a-dll-with-rust/index.html

## Project layout

This example consists of three Rust projects:

- `targetlib` - A library we want to proxy that exports a function
  named `add` (produces `targetlib.dll`, which we will later rename
  to `targetlib_orig.dll`)
- `evil` - A library that will pretend to be `targetlib` via library
  forwarding. Library forwarding is enabled at compile time using the
  `build.rs` script which configures the compiler to use the `test.def`
  file. Writes a silly message to the process' stderr when loaded (also
  produces a file named `targetlib.dll`)
- `app` - The application we want to load our `evil` library into.
  Calls targetlib's `add` function and writes the result to stdout
  (produces an executable named `app.exe`)

## Usage

Install git bash and rustup on Windows and execute the following in
git bash shell:

```sh
./run.sh
```

Here is what you should see:

```console
$ ./run.sh                                                     
(...)
>:) evil code loaded into: '(...)\app\target\debug\app.exe'
300
press enter to exit
```

## How it works

We compile `targetlib`, `evil`, and `app`. We rename `targetlib.dll`
to `targetlib_orig.dll` and place both the "evil" `targetlib.dll`
and `targetlib_orig.dll` in the same directory as `app.exe`. Since
Windows' library search order includes the executable's directory
(amazing), our "evil" library is loaded and its code executes.
The `add` function call is forwarded to the original targetlib.

## See also

You may find these references interesting as well :)

- ["How to use .def files to create forwarded exports in a Rust dll (cdylib)"][how-to-use-def-files-so]
- ["Re-export Shared Library Symbols from Other Library (OS X / POSIX)"][reexport-osx-so]
- [Rust build scripts manual][rust-build-scripts]
- ["public extern "C" symbols are not public in cdylib target"][rust-99411] (this bug made me sad)

[how-to-use-def-files-so]: https://stackoverflow.com/questions/78177063/how-to-use-def-files-to-create-forwarded-exports-in-a-rust-dll-cdylib
[reexport-osx-so]: https://stackoverflow.com/questions/20020715/re-export-shared-library-symbols-from-other-library-os-x-posix
[rust-build-scripts]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
[rust-99411]: https://github.com/rust-lang/rust/issues/99411
