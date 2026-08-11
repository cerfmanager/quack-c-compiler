main:
pushw r7
rrmovd r6, r7
irmovd $24, r5
sub r5, r6
irmovd $4, r5
rrmovd r7, r4
sub r5, r4
irmovd $2, r5
rrmmovd r5, r4
irmovd $4, r5
rrmovd r7, r4
sub r5, r4
rmrmovd r3, r4
dec r3
rrmmovd r3, r4
irmovd $4, r5
rrmovd r7, r4
sub r5, r4
rmrmovd r2, r4
irmovd $8, r5
rrmovd r7, r4
sub r5, r4
rrmmovd r2, r4
irmovd $8, r5
rrmovd r7, r4
sub r5, r4
rmrmovd r3, r4
neg r3
rrmmovd r3, r4
irmovd $8, r5
rrmovd r7, r4
sub r5, r4
rmrmovd r2, r4
irmovd $12, r5
rrmovd r7, r4
sub r5, r4
rrmmovd r2, r4
irmovd $12, r5
rrmovd r7, r4
sub r5, r4
rmrmovd r3, r4
neg r3
rrmmovd r3, r4
irmovd $12, r5
rrmovd r7, r4
sub r5, r4
rmrmovd r2, r4
irmovd $16, r5
rrmovd r7, r4
sub r5, r4
rrmmovd r2, r4
irmovd $16, r5
rrmovd r7, r4
sub r5, r4
rmrmovd r3, r4
bitcomp r3
rrmmovd r3, r4
irmovd $16, r5
rrmovd r7, r4
sub r5, r4
rmrmovd r2, r4
irmovd $20, r5
rrmovd r7, r4
sub r5, r4
rrmmovd r2, r4
irmovd $20, r5
rrmovd r7, r4
sub r5, r4
rmrmovd r3, r4
bitcomp r3
rrmmovd r3, r4
irmovd $20, r5
rrmovd r7, r4
sub r5, r4
rmrmovd r2, r4
irmovd $24, r5
rrmovd r7, r4
sub r5, r4
rrmmovd r2, r4
irmovd $24, r5
rrmovd r7, r4
sub r5, r4
rmrmovd r3, r4
dec r3
rrmmovd r3, r4
irmovd $24, r5
rrmovd r7, r4
sub r5, r4
rmrmovd r0, r4
rrmovd r7, r6
popw r7
halt
