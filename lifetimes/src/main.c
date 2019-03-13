
int *return_a_pointer(int left, int *x, int *y) {
  return left ? x + 1 : y + 1; 
}

int main() {
  int *x = (int *)malloc(16);
  int *y = (int *)malloc(16);

  int *z = return_a_pointer(1, x, y);
  // No idea whether x or y are okay to free later on.
}
