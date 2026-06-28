main:
    irmovd $0x0101, r0
    irmovd  $-50, r1
    add r0, r1
    shl r1, $2
    rrmovd r1, r0
    halt
