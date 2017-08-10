# graphql-idl-parser

A parser for GraphQL IDL files! This repository contains the code for both the Rust implementation,
as well as the FFI dylib implementation, which allows you to generate a library for use in C projects
(and other languages that play nice with C). See https://github.com/gjtorikian/graphql-idl-parser-ruby
for a Ruby implementation!

[![Build Status](https://travis-ci.org/gjtorikian/graphql-idl-parser.svg?branch=master)](https://travis-ci.org/gjtorikian/graphql-idl-parser)

## Why?

Two reasons:

* I have plenty of experience doing Bison/LALR grammars in C, and I desperately wanted to learn some Rust.
* Most of the GraphQL tooling, I'm sorry to say, appears to be in JavaScript. While that's cool, I
really believe that when it comes to creating a tool ecosystem, base implementations ought to be in
a language that can be consumed by other projects. In other words, the world doesn't need different GraphQL
IDL parsers in Node, and Ruby, and Python, and Java, and Scala. The base format should be in something
like C/C++/Rust, and then those higher level languages ought to make use of that one foundational library.
This ensures consistency and correctness, no matter what your backend is written in.

## Usage

Add this to your `Cargo.toml`:

``` toml
[dependencies]
graphql-idl-parser = "^0.1"
```

and this to your crate root:

``` rust
extern crate graphql_idl_parser;
```

After that, simply feed in a GraphQL IDL string to `gqlidl::parse_schema`, like:

``` rust
let definitions = gqlidl::parse_schema(contents.as_str()).unwrap();
for def in definitions {
  // ...
}
```

Note that this library does _not_ validate the format if your IDL. If you have multiple types with
the same name, for example, they will be happily consumed as unique types. However, the parser will
become irate if it comes across a malformed or unknown token.

## Todo:

- [ ] More documentation?
- [ ] Support directives in more places
- [ ] Support more of the IDL besides just types
