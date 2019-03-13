
// My C code that is definitely broken.
int *return_a_stack_pointer() {
  int x = 2;
  int *y = &x;
  return y;
}

int main() {}
