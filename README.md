## Usage
```bash
decompress_cap <YOUR_CAPABILITY>
```

## Installation

```bash
❯ git clone git@github.com:anti-entropy123/decompress_cap.git
❯ cd decompress_cap
❯ git submodule init && git submodule update
```

## Example

```bash
❯ cargo run -- 0xd1170000000140060000000000093d3e
Decompressing pesbt = d117000000014006, cursor = 0000000000093d3e
Permissions: 0x117 (CHERI_PERM_GLOBAL|CHERI_PERM_EXECUTE|CHERI_PERM_LOAD|CHERI_PERM_LOAD_CAP|CHERI_PERM_CCALL)
User Perms: 0xd (CHERI_PERM_SW0|CHERI_PERM_SW2|CHERI_PERM_SW3)
Base:   0x0000000000000000
Offset: 0x0000000000093d3e
Cursor: 0x0000000000093d3e
Length: 0x0000004000000000
Top:    0x0000004000000000
Sealed: false
Valid decompress: yes
```