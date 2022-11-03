#include <stdio.h>
#include <string.h>

float area(const char*, size_t, float, float);

int main(int argc, char * argv[])
{
  float a = 3;
  float b = 4;
  switch (argc) {
    case 1:
      printf("Missing name\n");
      break;
  case 2:
    printf("name: %s\n", argv[1]);
    printf("area: %f\n", area(argv[1], strlen(argv[1]), a, b));
    break;
  default:
    printf("Usage: %s <name>\n", argv[0]);
  }
  return 0;
}
