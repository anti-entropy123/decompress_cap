#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>

#include "../cheri-compressed-cap/cheri_compressed_cap_128.h"

void cc128_decompress(uint64_t pesbt, uint64_t cursor, bool tag, cc128_cap_t *result);
int32_t cc128_perms(cc128_cap_t *result);
int32_t cc128_uperms(cc128_cap_t *result);
bool cc128_is_sealed(cc128_cap_t *result);