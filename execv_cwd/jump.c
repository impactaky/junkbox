#include <unistd.h>

int main(int argc, char *argv[]) {
    char *args[] = {"mimiced!!!", NULL};
    char *env[] = {NULL};
    execve(argv[1], args, env);
    perror("execve");
    return 1;
}