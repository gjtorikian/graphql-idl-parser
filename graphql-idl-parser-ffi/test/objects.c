#include "clar.h"
#include "clar_test.h"
#include "helpers.h"
#include <stdio.h>
#include <string.h>

#include "gql-idl-parser.h"

static char *fixture;

void test_objects__initialize(void)
{
  global_test_counter++;
}

void test_objects__cleanup(void)
{
  if (fixture != NULL) {
    free(fixture);
  }
}

void test_objects__inline(void)
{
  fixture = read_fixture("objects.graphql");

  GraphQLTypes* types = NULL;
  size_t types_len = 0;
  uint8_t err;

  err = gqlidl_parse_schema(fixture, &types, &types_len);

  cl_assert_equal_i(err, 0);

  cl_assert_equal_s(types[0].typename, "object");
  cl_assert_equal_s("", types[0].object.description);
  cl_assert_equal_s("CodeOfConduct", types[0].object.name);

  cl_assert_equal_s(types[1].typename, "object");
  cl_assert_equal_s("The Code of Conduct for a repository", types[1].object.description);
  cl_assert_equal_s("CodeOfConduct", types[1].object.name);

  cl_assert_equal_s(types[2].typename, "object");
  cl_assert_equal_s("", types[2].object.description);
  cl_assert_equal_s("PushAllowance", types[2].object.name);
  cl_assert_equal_i(1, types[2].object.implements.length);
  cl_assert_equal_s("Node", types[2].object.implements.values[0]);

  cl_assert_equal_s(types[3].typename, "object");
  cl_assert_equal_s("", types[3].object.description);
  cl_assert_equal_s("Release", types[3].object.name);
  cl_assert_equal_i(2, types[3].object.implements.length);
  cl_assert_equal_s("Node", types[3].object.implements.values[0]);
  cl_assert_equal_s("UniformResourceLocatable", types[3].object.implements.values[1]);
}
