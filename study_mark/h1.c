#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>  // 添加这个头文件以支持文件锁

int main() {
    int p1, p2, i;

    while ((p1 = fork()) == -1);  // 创建子进程p1
    if (p1 == 0) {
        lockf(1, 1, 0);  // 加锁，1表示stdout
        for (i = 0; i < 10; i++)
            printf("daughter %d\n", i);
        lockf(1, 0, 0);  // 解锁
    } else {
        while ((p2 = fork()) == -1);  // 创建子进程p2
        if (p2 == 0) {
            lockf(1, 1, 0);  // 加锁
            for (i = 0; i < 10; i++)
                printf("son %d\n", i);
            lockf(1, 0, 0);  // 解锁
        } else {
            lockf(1, 1, 0);  // 加锁
            for (i = 0; i < 10; i++)
                printf("parent %d\n", i);
            lockf(1, 0, 0);  // 解锁
        }
    }
    return 0;  // 添加返回值
}
