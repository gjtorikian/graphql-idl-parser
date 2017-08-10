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

  cl_assert_equal_s(types[4].typename, "object");
  cl_assert_equal_s("The Code of Conduct for a repository", types[4].object.description);
  cl_assert_equal_s("CodeOfConduct", types[4].object.name);
  cl_assert_equal_i(1, types[4].object.fields.length);
  cl_assert_equal_s("body", types[4].object.fields.values[0].name);
  cl_assert_equal_s("", types[4].object.fields.values[0].description);
  cl_assert_equal_b(false, types[4].object.fields.values[0].deprecated);

  cl_assert_equal_s(types[5].typename, "object");
  cl_assert_equal_s("The Code of Conduct for a repository", types[5].object.description);
  cl_assert_equal_s("CodeOfConduct", types[5].object.name);
  cl_assert_equal_i(1, types[5].object.fields.length);
  cl_assert_equal_s("body", types[5].object.fields.values[0].name);
  cl_assert_equal_s("The body of the CoC", types[5].object.fields.values[0].description);
  cl_assert_equal_b(false, types[5].object.fields.values[0].deprecated);

  cl_assert_equal_s(types[6].typename, "object");
  cl_assert_equal_s("key", types[6].object.fields.values[0].name);
  cl_assert_equal_s("String", types[6].object.fields.values[0].type_info.name);
  cl_assert_equal_s("!", types[6].object.fields.values[0].type_info.type_info);

  cl_assert_equal_s(types[7].typename, "object");
  cl_assert_equal_s("edges", types[7].object.fields.values[0].name);
  cl_assert_equal_s("CommitCommentEdge", types[7].object.fields.values[0].type_info.name);
  cl_assert_equal_s("[]", types[7].object.fields.values[0].type_info.type_info);

  cl_assert_equal_s(types[8].typename, "object");
  cl_assert_equal_s("suggestedReviewers", types[8].object.fields.values[0].name);
  cl_assert_equal_s("SuggestedReviewer", types[8].object.fields.values[0].type_info.name);
  cl_assert_equal_s("[]!", types[8].object.fields.values[0].type_info.type_info);

  cl_assert_equal_s(types[9].typename, "object");
  cl_assert_equal_s("viewerCannotUpdateReasons", types[9].object.fields.values[0].name);
  cl_assert_equal_s("CommentCannotUpdateReason", types[9].object.fields.values[0].type_info.name);
  cl_assert_equal_s("[!]!", types[9].object.fields.values[0].type_info.type_info);

  cl_assert_equal_s(types[10].typename, "object");
  cl_assert_equal_s("followers", types[10].object.fields.values[0].name);
  cl_assert_equal_s("FollowerConnection", types[10].object.fields.values[0].type_info.name);
  cl_assert_equal_s("!", types[10].object.fields.values[0].type_info.type_info);
  cl_assert_equal_i(2, types[10].object.fields.values[0].arguments.length);
  cl_assert_equal_s("Returns the elements in the list that come after the specified global ID.", types[10].object.fields.values[0].arguments.values[0].description);
  cl_assert_equal_s("after", types[10].object.fields.values[0].arguments.values[0].name);
  cl_assert_equal_s("String", types[10].object.fields.values[0].arguments.values[0].type_info.name);
  cl_assert_equal_s("", types[10].object.fields.values[0].arguments.values[0].type_info.type_info);
  cl_assert_equal_s("Returns the first _n_ elements from the list.", types[10].object.fields.values[0].arguments.values[1].description);
  cl_assert_equal_s("first", types[10].object.fields.values[0].arguments.values[1].name);
  cl_assert_equal_s("Int", types[10].object.fields.values[0].arguments.values[1].type_info.name);
  cl_assert_equal_s("!", types[10].object.fields.values[0].arguments.values[1].type_info.type_info);

  cl_assert_equal_s(types[11].typename, "object");
  cl_assert_equal_s("User", types[11].object.name);
  cl_assert_equal_b(true, types[11].object.fields.values[0].deprecated);
  cl_assert_equal_s("", types[11].object.fields.values[0].deprecation_reason);

  cl_assert_equal_s(types[12].typename, "object");
  cl_assert_equal_s("User", types[12].object.name);
  cl_assert_equal_b(true, types[12].object.fields.values[0].deprecated);
  cl_assert_equal_s("Exposed database IDs will eventually be removed in favor of global Relay IDs.", types[12].object.fields.values[0].deprecation_reason);

}
