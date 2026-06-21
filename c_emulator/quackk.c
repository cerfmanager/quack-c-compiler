/*
full emulator rewwrite to implement ncuses cli, signed values




TODO:
-negative number support
-better way to run the program , maybe a gui ?
-state of memory and all the registers
-add more registers
-larger number support ?

 */




#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

// definitions
// size of memory array
#define MEM_SIZE 8192
// maximum accecible memory address become 0x2000

//memory location definitions
#define CODE_START  0x0000
#define CODE_END    0x0FFE
#define DATA_START  0x0FFF
#define DATA_END    0x1E4F
//TODO: redo the io position in memory
//#define IO_START    0x0BE0
//#define IO_END      0x0BFF
#define STACK_START 0x1E50
#define STACK_END   0x1FFF

#define SP_INIT     0x2000   /* stack pointer starts one past 0x0FFF */

/* I/O registers (memory-mapped)
TODO: for io maybe implement the output in another way ???
 */
// #define IO_KEY     0x0BE0  /* read: ASCII key; consumes it */
// #define IO_STATUS  0x0BE1  /* read: 1 if key available else 0 */
// #define IO_PUTCHAR 0x0BE2  /* write: prints one character */
// #define IO_CLEAR   0x0BE3  /* write: clears the terminal */
// #define IO_TICK    0x0BE4  /* read: low 8 bits of tick counter */




/* =========================
    Opcodes
   ========================= */

/* Data movement */
// TODO: implement double word
#define OP_RRMOVW  0x01  /* rrmovw  ra <- rb          (rb in b2)  increments pc by 4 */
#define OP_IRMOVB  0x02  /* irmovb  ra <- imm8        (imm8 in b3) increments pc by 4 */
#define OP_MRMOVW  0x03  /* mrmovw  ra <- mem16[imm16] increments pc by 4 */
#define OP_RMMOVW  0x04  /* rmmovw  mem16[imm16] <- ra increments pc by 4 */
#define OP_IRMOVW  0x05  /* irmovw  ra <- imm16       (imm16 in b2,b3) increments pc by 4 */

/* Byte load/store (used by maze) */
#define OP_MRMOVB  0x06  /* mrmovb  ra <- mem8[imm16] increments pc by 4 */
#define OP_RMMOVB  0x07  /* rmmovb  mem8[imm16] <- low8(ra) increments pc by 4 */
#define OP_MRMOVBR 0x08  /* mrmovbR ra <- mem8[Rb]     (rb in b2) increments pc by 4 */
#define OP_RMMOVBR 0x09  /* rmmovbR mem8[Rb] <- low8(ra) increments pc by 4 */

/* ALU */
#define OP_ADDW    0x10  /* addw ra <- rb (rb in b2) increments pc by 4 */
#define OP_SUBW    0x11  /* subw ra <- rb (rb in b2) increments pc by 4 */
#define OP_INCW    0x12  /* increase ra by 1 */
#define OP_DECW    0x13  /* decrease ra by 1 */
#define OP_CLRW    0x14  /* clears value in ra */
#define OP_CMPW    0x15  /* sets ZF increments pc by 4 */

/* Control flow */
#define OP_JMP     0x20  /* jump to adress stored in b2 & b3 doesnt change pc */
#define OP_JE      0x21  /* jump if ZF==1 doesnt change pc */
#define OP_JNE     0x22  /* jump if ZF==0 doesnt change pc */
#define OP_HALT    0x23  /* sets halted flag to 1 (cpu stops on the next cycle) doesnt change pc */

/* Stack / procedures */
#define OP_PUSHW   0x30  /* pushes value of ra onto the stack and decreases stack pointer by 2 doesnt change pc */
#define OP_POPW    0x31  /* pops value from the stack onto ra increases stack pointer by 2 doesnt change pc */
#define OP_CALL    0x32  /* pushes the current pc onto the stack and then jump to address stored in b2 & b3 , changes pc to jump address*/
#define OP_RET     0x33  /* pops the address of the calling pc from the stack and jumps to it , changes to pc to caller address */

/* Output instructions */
#define OP_OUTC    0x40  /* displays the content of ra as a character*/
