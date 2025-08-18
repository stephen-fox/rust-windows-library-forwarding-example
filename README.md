# Windows import forwarding example in Rust

Note: This code is for educational purposes only.

This repository demonstrates proxying a Windows dynamic library using
import forwarding in Rust. It relies on the Windows compiler's
[module-definition (DEF)][def] file format. For a full explanation
of this technique, please refer to ["dll-hijack-by-proxying"][tothi-repo]
by GitHub user [tothi][tothi].


[def]: https://learn.microsoft.com/en-us/cpp/build/exporting-from-a-dll-using-def-files?view=msvc-170
[tothi-repo]: https://github.com/tothi/dll-hijack-by-proxying
[tothi]: https://github.com/tothi

## Project layout

- `targetlib` - The library we want to proxy
- `evil` - A library that will pretend to be `targetlib` via import forwarding
- `app` - The application we want to load our `evil` library into

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
>:) evil code loaded into: 'C:\Users\sfoxjr\src\windows-def-forwarder-test\app\t
arget\debug\app.exe'
300
press enter to exit
```
