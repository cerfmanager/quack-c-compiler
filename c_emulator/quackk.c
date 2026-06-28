/*
full emulator rewwrite to implement ncuses cli, signed values




TODO:
-negative number support
-better way to run the program , maybe a gui ?
-documentation on all the opcodes

 */

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// definitions
// size of memory array
#define MEM_SIZE 8192
// maximum accecible memory address become 0x2000

// memory location definitions
#define CODE_START 0x0000
#define CODE_END 0x0FFE
#define DATA_START 0x0FFF
#define DATA_END 0x1E4F
// TODO: redo the io position in memory
// #define IO_START    0x0BE0
// #define IO_END      0x0BFF
#define STACK_START 0x1E50
#define STACK_END 0x1FFF

#define SP_INIT 0x2000 /* stack pointer starts one past 0x0FFF */

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
#define OP_RRMOVB 0x01
#define OP_RRMOVW 0x02
#define OP_RRMOVD 0x03

// immediate to register register in ra and value in b2 b3 b4 b5
#define OP_IRMOVB 0x04
#define OP_IRMOVW 0x05
#define OP_IRMOVD 0x06

// memory to register
#define OP_MRMOVB 0x07
#define OP_MRMOVW 0x08
#define OP_MRMOVD 0x09

// register to memory
#define OP_RMMOVB 0x0A
#define OP_RMMOVW 0x0B
#define OP_RMMOVD 0x0C

// arithemtic
#define OP_ADD 0x0D
#define OP_SUB 0x0E
#define OP_INC 0x0F
#define OP_DEC 0x10
#define OP_CLR 0x11

// logical
#define OP_RAND 0x12
#define OP_ROR 0x13
#define OP_RXOR 0x14

#define OP_IAND 0x15
#define OP_IOR 0x16
#define OP_IXOR 0x17

// logical shift (0 fill)
// TODO:implement shifting by a value stored in a register
#define OP_SHL 0x18
#define OP_SHR 0x19
// arithmetic shift (sign fill)
#define OP_SAR 0x1A
#define OP_SAL 0x1B

// control procedures
#define OP_CMP 0x1C
#define OP_JMP 0x1D
#define OP_JE 0x1E
#define OP_JNE 0x1F
#define OP_JG 0x20
#define OP_JL 0x21
#define OP_JGE 0x22
#define OP_JLE 0x23
#define OP_JA 0x24
#define OP_JB 0x25

/* Stack / procedures */
#define OP_PUSHW                                                               \
  0x26 /* pushes value of ra onto the stack and decreases stack pointer by 2   \
          doesnt change pc */
#define OP_POPW                                                                \
  0x27 /* pops value from the stack onto ra increases stack pointer by 2       \
          doesnt change pc */
#define OP_CALL                                                                \
  0x28 /* pushes the current pc onto the stack and then jump to address stored \
          in b2 & b3 , changes pc to jump address*/
#define OP_RET                                                                 \
  0x29 /* pops the address of the calling pc from the stack and jumps to it ,  \
          changes to pc to caller address */

/* Output instructions */
#define OP_OUTC 0x2A /* displays the content of ra as a character*/

/*stop program execution */
#define OP_HALT 0x2B

static uint8_t memory[MEM_SIZE];

typedef struct {
  uint16_t pc;   /* program counter (address of next instruction) */
  uint16_t sp;   /* stack pointer */
  uint32_t r[6]; /* R0..R5 */
  uint8_t zf;    /* zero flag */
  uint8_t cf;    // carry flag(unsigned)
  uint8_t of;    // overflow flag(signed)
  uint8_t sf;    // sign flag
  int halted;    /* 1 if HALT executed */
} cpu_t;

/* One decoded instruction (6 bytes). */
typedef struct {
  uint8_t op;
  uint8_t ra;
  uint8_t b2;
  uint8_t b3;
  uint8_t b4;
  uint8_t b5;
} instr_t;

// helper functions
// byte word and double conversions
// =================
static uint16_t u16_from_le(uint8_t lo, uint8_t hi) {
  return (uint16_t)(lo | ((uint16_t)hi << 8));
}

static u_int32_t u32_from_le(uint16_t lo, uint16_t hi) {
  return (uint32_t)(lo | ((uint32_t)hi << 16));
}

static u_int32_t u32_from_u8(uint8_t lo, uint8_t mid1, uint8_t mid2,
                             uint8_t hi) {
  return u32_from_le(u16_from_le(lo, mid1), u16_from_le(mid2, hi));
}
// =================
// error handling
// =================
static void memory_error(uint16_t addr) {
  printf("MEMORY ERROR at 0x%04X\n", addr);
  exit(1);
}

static void die(const char *msg) {
  printf("%s\n", msg);
  exit(1);
}
// =================
// memory checks
// =================
static void byte_memory_check(uint16_t address) {
  if (address >= MEM_SIZE) {
    memory_error(address);
  }
}

static void word_memory_check(uint16_t address) {
  if (address >= MEM_SIZE || address >= MEM_SIZE + 1) {
    memory_error(address);
  }
}

static void double_memory_check(uint16_t address) {
  if (address >= MEM_SIZE || address >= MEM_SIZE + 1 ||
      address >= MEM_SIZE + 2 || address >= MEM_SIZE + 3) {
    memory_error(address);
  }
}
// =================

// memory operations
// =================
static uint8_t mem_read8(uint16_t addr) {
  byte_memory_check(addr);
  return memory[addr];
}

static void mem_write8(uint16_t addr, uint8_t v) {
  byte_memory_check(addr);
  memory[addr] = v;
}

static uint16_t mem_read16(uint16_t addr) {
  word_memory_check(addr);
  return memory[addr] | (memory[addr + 1] << 8);
}

static void mem_write16(uint16_t addr, uint16_t v) {
  word_memory_check(addr);
  memory[addr] = v & 0xFF;
  memory[addr + 1] = (v >> 8) & 0xFF;
}

static uint32_t mem_read32(uint16_t addr) {
  double_memory_check(addr);
  return memory[addr] | (memory[addr + 1] << 8) | (memory[addr + 2] << 16) |
         (memory[addr + 3] << 24);
}

static void mem_write32(uint16_t addr, uint32_t v) {
  double_memory_check(addr);
  memory[addr] = v & 0xFF;
  memory[addr + 1] = (v >> 8) & 0xFF;
  memory[addr + 2] = (v >> 16) & 0xFF;
  memory[addr + 3] = (v >> 24) & 0xFF;
}
// =================

// cpu operations
// =================
static void cpu_reset(cpu_t *cpu) {
  cpu->pc = 0;
  cpu->sp = SP_INIT;
  cpu->r[0] = cpu->r[1] = cpu->r[2] = cpu->r[3] = 0;
  cpu->zf = 0;
  cpu->sf = 0;
  cpu->of = 0;
  cpu->cf = 0;
  cpu->halted = 0;
}
// cpu is passed by reference, this mean state DOES change
static void set_zf(uint8_t r, cpu_t *cpu) { cpu->zf = cpu->r[r] == 0; }

static void check_reg(uint8_t r) {
  if (r > 5)
    die("Invalid register index (valid: 0..3)");
}

static instr_t fetch(cpu_t *cpu) {
  instr_t in;
  in.op = mem_read8(cpu->pc + 0);
  in.ra = mem_read8(cpu->pc + 1);
  in.b2 = mem_read8(cpu->pc + 2);
  in.b3 = mem_read8(cpu->pc + 3);
  in.b4 = mem_read8(cpu->pc + 4);
  in.b5 = mem_read8(cpu->pc + 5);
  return in;
}
// =================

static void mem_clear(void) {
  for (int i = 0; i < MEM_SIZE; i++)
    memory[i] = 0;
}

static void load_program(const char *path) {
  FILE *f = fopen(path, "rb");
  if (!f)
    die("Could not open program file");

  int c;
  int addr = 0;
  while ((c = fgetc(f)) != EOF) {
    if (addr >= MEM_SIZE)
      die("Program too large for memory");
    memory[addr++] = (uint8_t)c;
  }
  fclose(f);
}

static void cpu_step(cpu_t *cpu, int debug) {
  (void)debug;

  if (cpu->halted)
    return;

  instr_t in = fetch(cpu);

  /* In debug mode, print a simple trace */
  if (debug) {
      printf("PC=%04X OP=%02X R0=%04X R1=%04X R2=%04X R3=%04X ZF=%u SF=%u OF=%u CF=%u SP=%04X\n",
             cpu->pc, in.op, cpu->r[0], cpu->r[1], cpu->r[2], cpu->r[3], cpu->zf, cpu->sf, cpu->of, cpu->cf, cpu->sp);
  }


  uint16_t mem_addr;
  switch (in.op) {

  // REGISTER TO REGISTER MOV
  case OP_RRMOVB:
    check_reg(in.ra);
    check_reg(in.b3);
    cpu->r[in.ra] = cpu->r[in.b3];
    cpu->pc += 6;
    break;

  case OP_RRMOVW:
    check_reg(in.ra);
    check_reg(in.b3);
    cpu->r[in.ra] = cpu->r[in.b3];
    cpu->pc += 6;
    break;

  case OP_RRMOVD:
    check_reg(in.ra);
    check_reg(in.b3);
    cpu->r[in.ra] = cpu->r[in.b3];
    cpu->pc += 6;
    break;

  // IMMEDIATE TO REGISTER MOV
  case OP_IRMOVB:
    check_reg(in.ra);
    cpu->r[in.ra] = (u_int32_t)in.b2;
    cpu->pc += 6;
    break;

  case OP_IRMOVW:
    check_reg(in.ra);
    cpu->r[in.ra] = (u_int32_t)u16_from_le(in.b2, in.b3);
    cpu->pc += 6;
    break;

  case OP_IRMOVD:
    check_reg(in.ra);
    cpu->r[in.ra] = u32_from_u8(in.b2, in.b3, in.b4, in.b5);
    cpu->pc += 6;
    break;

  // the implementation of IO will change the representation of this,

  // MEMORY TO REGISTER MOV
  // the memory check is done in the function itself
  case OP_MRMOVB:
    check_reg(in.ra);
    mem_addr = u32_from_u8(in.b2, in.b3, in.b4, in.b5);
    cpu->r[in.ra] = mem_read8(mem_addr);
    cpu->pc += 6;
    break;

  case OP_MRMOVW:
    check_reg(in.ra);
    mem_addr = u32_from_u8(in.b2, in.b3, in.b4, in.b5);
    cpu->r[in.ra] = mem_read16(mem_addr);
    cpu->pc += 6;
    break;

  case OP_MRMOVD:
    check_reg(in.ra);
    mem_addr = u32_from_u8(in.b2, in.b3, in.b4, in.b5);
    cpu->r[in.ra] = mem_read32(mem_addr);
    cpu->pc += 6;
    break;

  // REGISTER TO MEMORY MOV
  // the memory check is done in the function itself
  case OP_RMMOVB:
    check_reg(in.ra);
    mem_addr = u32_from_u8(in.b2, in.b3, in.b4, in.b5);
    mem_write8(mem_addr, (uint8_t)cpu->r[in.ra]);
    cpu->pc += 6;
    break;

  case OP_RMMOVW:
    check_reg(in.ra);
    mem_addr = u32_from_u8(in.b2, in.b3, in.b4, in.b5);
    mem_write16(mem_addr, (uint16_t)cpu->r[in.ra]);
    cpu->pc += 6;
    break;

  case OP_RMMOVD:
    check_reg(in.ra);
    mem_addr = u32_from_u8(in.b2, in.b3, in.b4, in.b5);
    mem_write32(mem_addr, cpu->r[in.ra]);
    cpu->pc += 6;
    break;

  // ARITHMETIC OPERATIONS
  case OP_ADD:
    check_reg(in.ra);
    check_reg(in.b2);
    cpu->r[in.b2] += cpu->r[in.ra];
    set_zf(in.b2, cpu);
    cpu->pc += 6;
    break;

  case OP_SUB:
    check_reg(in.ra);
    check_reg(in.b2);
    cpu->r[in.b2] -= cpu->r[in.ra];
    set_zf(in.b2, cpu);
    cpu->pc += 6;
    break;

  case OP_DEC:
    check_reg(in.ra);
    cpu->r[in.ra]--;
    cpu->pc += 6;
    break;

  case OP_INC:
    check_reg(in.ra);
    cpu->r[in.ra]++;
    cpu->pc += 6;
    break;

  case OP_CLR:
    check_reg(in.ra);
    cpu->r[in.ra] = 0;
    cpu->pc += 6;
    break;

    // LOGICAL OPERATIONS

  case OP_IAND:
    check_reg(in.ra);
    u_int32_t val_and = u32_from_u8(in.b2, in.b3, in.b4, in.b5);
    cpu->r[in.ra] = cpu->r[in.ra] & val_and;
    set_zf(in.ra, cpu);
    cpu->pc += 6;
    break;

  // this isnt random , its register and
  case OP_RAND:
    check_reg(in.ra);
    check_reg(in.b2);
    cpu->r[in.b2] = cpu->r[in.ra] & cpu->r[in.b2];
    set_zf(in.ra, cpu);
    cpu->pc += 6;
    break;

  case OP_IOR:
    check_reg(in.ra);
    u_int32_t val_or = u32_from_u8(in.b2, in.b3, in.b4, in.b5);
    cpu->r[in.ra] = cpu->r[in.ra] | val_or;
    set_zf(in.ra, cpu);
    cpu->pc += 6;
    break;

  case OP_ROR:
    check_reg(in.ra);
    check_reg(in.b2);
    cpu->r[in.b2] = cpu->r[in.ra] | cpu->r[in.b2];
    set_zf(in.ra, cpu);
    cpu->pc += 6;
    break;

  case OP_IXOR:
    check_reg(in.ra);
    u_int32_t val_xor = u32_from_u8(in.b2, in.b3, in.b4, in.b5);
    cpu->r[in.ra] = cpu->r[in.ra] ^ val_xor;
    set_zf(in.ra, cpu);
    cpu->pc += 6;
    break;

  case OP_RXOR:
    check_reg(in.ra);
    check_reg(in.b2);
    cpu->r[in.b2] = cpu->r[in.ra] ^ cpu->r[in.b2];
    set_zf(in.ra, cpu);
    cpu->pc += 6;
    break;

  case OP_SHL:
    check_reg(in.ra);
    uint32_t lls = u32_from_u8(in.b2, in.b3, in.b4, in.b5);
    cpu->r[in.ra] = cpu->r[in.ra] << lls;
    cpu->pc += 6;
    break;

  case OP_SHR:
    check_reg(in.ra);
    uint32_t rls = u32_from_u8(in.b2, in.b3, in.b4, in.b5);
    cpu->r[in.ra] = cpu->r[in.ra] >> rls;
    cpu->pc += 6;
    break;

  case OP_SAL:
    check_reg(in.ra);
    uint32_t las = u32_from_u8(in.b2, in.b3, in.b4, in.b5);
    cpu->r[in.ra] = cpu->r[in.ra] << las;
    cpu->pc += 6;
    break;

  case OP_SAR:
    check_reg(in.ra);
    uint32_t ras = u32_from_u8(in.b2, in.b3, in.b4, in.b5);
    int32_t toshift = (int32_t)cpu->r[in.ra];
    cpu->r[in.ra] = toshift >> ras;
    cpu->pc += 6;
    break;

    // control flow

  case OP_CMP:
    check_reg(in.ra);
    check_reg(in.b2);
    uint32_t cmp_a = cpu->r[in.ra];
    uint32_t cmp_b = cpu->r[in.b2];
    uint32_t cmp_result = cmp_a - cmp_b;
    cpu->zf = (cmp_result == 0);
    cpu->sf = (cmp_result >> 31) & 1;
    cpu->cf = (cmp_a < cmp_b);
    cpu->of = ((cmp_a ^ cmp_b) & (cmp_a ^ cmp_result)) >> 31 & 1;
    cpu->pc += 6;
    break;

  case OP_JMP:
    cpu->pc = u16_from_le(in.b2, in.b3);
    break;

  case OP_JE:
    if (cpu->zf) {
      cpu->pc = u16_from_le(in.b2, in.b3);
    } else {
      cpu->pc += 6;
    }
    break;

  case OP_JNE:
    if (!cpu->zf) {
      cpu->pc = u16_from_le(in.b2, in.b3);
    } else {
      cpu->pc += 6;
    }
    break;

    // signed comparisons: rely on SF/OF from CMP, same logic x86 uses
  case OP_JG:
    if (!cpu->zf && cpu->sf == cpu->of) {
      cpu->pc = u16_from_le(in.b2, in.b3);
    } else {
      cpu->pc += 6;
    }
    break;

  case OP_JGE:
    if (cpu->sf == cpu->of) {
      cpu->pc = u16_from_le(in.b2, in.b3);
    } else {
      cpu->pc += 6;
    }
    break;

  case OP_JL:
    if (cpu->sf != cpu->of) {
      cpu->pc = u16_from_le(in.b2, in.b3);
    } else {
      cpu->pc += 6;
    }
    break;

  case OP_JLE:
    if (cpu->zf || cpu->sf != cpu->of) {
      cpu->pc = u16_from_le(in.b2, in.b3);
    } else {
      cpu->pc += 6;
    }
    break;

  // unsigned comparisons: rely on CF from CMP
  case OP_JA:
    if (!cpu->cf && !cpu->zf) {
      cpu->pc = u16_from_le(in.b2, in.b3);
    } else {
      cpu->pc += 6;
    }
    break;

  case OP_JB:
    if (cpu->cf) {
      cpu->pc = u16_from_le(in.b2, in.b3);
    } else {
      cpu->pc += 6;
    }
    break;

  case OP_HALT:
    cpu->halted = 1;
    break;
  case OP_PUSHW:
    check_reg(in.ra);
    cpu->sp -= 2;
    mem_write16(cpu->sp, cpu->r[in.ra]);
    cpu->pc += 6;
    break;
  case OP_POPW:
    check_reg(in.ra);
    cpu->r[in.ra] = mem_read16(cpu->sp);
    cpu->sp += 2;
    cpu->zf = cpu->r[in.ra] == 0;
    cpu->pc += 6;
    break;
  case OP_CALL:
    cpu->sp -= 2;
    mem_write16(cpu->sp, cpu->pc + 4);
    cpu->pc = u16_from_le(in.b2, in.b3);
    break;
  case OP_RET:
    cpu->pc = mem_read16(cpu->sp);
    cpu->sp += 2;
    break;

  default:
    char msg[30];
    snprintf(msg, sizeof(msg), "unknown opcode %02X",in.op);
    die(msg);
    break;

  }
}
static void cpu_run(cpu_t *cpu, int debug) {
  while (!cpu->halted) {
    cpu_step(cpu, debug);
    if (cpu->pc >= MEM_SIZE && !cpu->halted)
      die("PC out of bounds");
  }
}

int main(int argc, char **argv) {

  int debug = 0;
  if (argc < 2) {
    die("no specified path");
  }
  const char *path = argv[1];

  if (argc == 3) {
    if (strcmp(argv[2], "--debug") == 0)
      debug = 1;
  }

  mem_clear();

  load_program(path);

  cpu_t cpu;
  cpu_reset(&cpu);

  cpu_run(&cpu, debug);

  printf("\nHALT\n");
  printf("R0(final)=%04X\n", cpu.r[0]);
  return 0;
}
