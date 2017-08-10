#include "clar.h"
#include "clar_test.h"
#include "helpers.h"
#include <stdio.h>
#include <string.h>

#include "gql-idl-parser.h"

static char *fixture;

void test_unions__initialize(void)
{
  global_test_counter++;
}

void test_unions__cleanup(void)
{
  if (fixture != NULL) {
    free(fixture);
  }
}

void test_unions__inline(void)
{
  fixture = read_fixture("unions.graphql");

  GraphQLTypes* types = NULL;
  size_t types_len = 0;
  uint8_t err;

  err = gqlidl_parse_schema(fixture, &types, &types_len);

  cl_assert_equal_i(err, 0);

  cl_assert_equal_s(types[0].typename, "union");
  cl_assert_equal_s("ReferencedSubject", types[0].union_type.name);
  cl_assert_equal_s("Any referencable object", types[0].union_type.description);
  cl_assert_equal_i(2, types[0].union_type.values.length);
  cl_assert_equal_s("Issue", types[0].union_type.values.values[0]);
  cl_assert_equal_s("PullRequest", types[0].union_type.values.values[1]);
}
