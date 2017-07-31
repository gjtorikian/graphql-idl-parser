#include "clar.h"
#include "clar_test.h"
#include "helpers.h"
#include <stdio.h>
#include <string.h>

static char *fixture;
static char *result;

void test_basic__initialize(void)
{
  global_test_counter++;
}

void test_basic__cleanup(void)
{
  if (fixture != NULL) {
    free(fixture);
  }
}

void test_basic__inline(void)
{
  fixture = read_fixture("basic.graphql");

  cl_assert_equal_i(1, 1);
}
