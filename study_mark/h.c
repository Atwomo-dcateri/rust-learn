#include <stdio.h>
# include < stdlib.h>
main( )
{
    int p1,p2;
    while((p1 = fork()) == -1);        /*创建子进程p1*/
    if (p1 == 0)  
    {  
        printf("My ID is:%d", getpid()); 
        exit(0);
    } else {
        wait(0)
        printf("I know, My son's ID is:%d", p1());
        printf("My ID is: %d", getpid());
    }
}
