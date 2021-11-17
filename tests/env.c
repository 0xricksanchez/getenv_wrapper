#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>


int main(int argc, char **argv) {
    if(argc != 2) {
        printf("Usage: %s <ENV_VAR>\n", argv[0]);
        return -1;
    }
    
    char* env = getenv(argv[1]);
    if(setenv(argv[1], env, 1) != 0) { 
        printf("Failed to set %s with new data!\n", argv[1]);
        return -1;
    }
    
    printf("[>] Post-Hook: $%s has value -> ", argv[1]);
    printf("%s\n", env);

    return 0;
}
