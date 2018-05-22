# `edmunge`

Good news everybody! You can finally use your favorite inscrutable text editor as a scripting language!

`edmunge` is a recursive acronym that stands for Edmunge Mashes Until No Good, with Ed.

Briefly, `edmunge` lets you destructively edit text files using scripts filled with ed commands. What could go wrong?

For example:

```shell
$ cat document
My sweet document!
$ cat script.ed
#!/usr/bin/env edmunge
,s/!/?/
w
$ ./script.ed document
19
19
$ cat document
My sweet document?
```

## installation

```shell
$ cargo install edmunge
```

This was inspired by Julia Evan's [blog post on batch-editing files with ed](https://jvns.ca/blog/2018/05/11/batch-editing-files-with-ed/).

To improve your ed skills, I recommend the delightful [Ed Mastery](https://www.michaelwlucas.com/tools/ed) by Michael W. Lucas.
