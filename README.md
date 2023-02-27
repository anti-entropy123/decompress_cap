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
Permissions: 0x117
User Perms: 0xd
Base:   0x0000000000000000
Offset: 0x0000000000093d3e
Cursor: 0x0000000000093d3e
Length: 0x0000004000000000
Top:    0x0000004000000000
Sealed: false
Valid decompress: yes
```