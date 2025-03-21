#include <unistd.h>
#include <sys/syscall.h>

int main() {
    // Mount proc and sysfs
    syscall(SYS_mount, "proc", "/proc", "proc", 0, NULL);
    syscall(SYS_mount, "sysfs", "/sys", "sysfs", 0, NULL);

    // Start a shell
    char *argv[] = {"/bin/sh", NULL};
    syscall(SYS_execve, argv[0], argv, NULL);

    return 0;
}
