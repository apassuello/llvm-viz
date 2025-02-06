#include "math.h"

extern int32_t add(int32_t a, int32_t b);
extern int32_t mult(int32_t a, int32_t b) {
        return a * b;
}

int main(int argc, char *argv[])
{
        return add(1,1);
}
