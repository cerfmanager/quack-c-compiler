main:
pushw r7
rrmovd r6, r7
irmovd $0, r5
sub r5, r6
irmovd $55, r0
rrmovd r7, r6
popw r7
halt
