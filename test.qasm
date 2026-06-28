main:
    irmovd $-5, r0
    irmovd $10, r1
    cmp r0, r1
    jg .sex
    halt
.sex:
    irmovd $20, r0
    jmp .stop
.stop:
    halt
