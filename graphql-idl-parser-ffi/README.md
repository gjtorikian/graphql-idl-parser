# C API for graphql-idl-parser

This is an FFI binding to the `graphql-idl-parser` crate, which allows you to use
that crate in your C project.

The header file (`includes/gql-idl-parser.h`) serves as the primary API documentation of
this library. T

## Usage

There are examples on how to use this in C in the `example` and `test` sub-directories.

Simply running `cargo build --release` should generate the `.so` or `.dylib` for you.

Assuming you have a C compiler, then this should work to run the example:

``` console
$ git clone git://github.com/gjtorikian/graphql-idl-parser
$ ./compile
```

Here's a general usage example:

``` c
#include "gql-idl-parser.h"

int main()
{
  GraphQLTypes* types = NULL;
  size_t types_len = 0;
  uint8_t err;

  err = gqlidl_parse_schema("# Yeah yeah\nscalar DateTime\ntype OMG {}", &types, &types_len);

  if (err > 0) {
    printf("Error: Return code %d", err);
    exit(err);
  }

  for (size_t i = 0; i < types_len; i++) {
    // ...
  }
}
```

## TODO:

- [ ] Run Valgrind
