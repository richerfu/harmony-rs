#include "stdio.h"
//#include "add.h"

extern unsigned add(unsigned a,unsigned b);

int main() {
    unsigned result = add(1,2);
    printf("%d",result);
    return 0;
}