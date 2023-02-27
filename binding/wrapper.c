#include "wrapper.h"

void cc128_decompress(uint64_t pesbt, uint64_t cursor, bool tag, cc128_cap_t *result)
{
    cc128_decompress_mem(pesbt, cursor, false, result);
}

int32_t cc128_perms(cc128_cap_t *result) { return cc128_get_perms(result); }

int32_t cc128_uperms(cc128_cap_t *result) { return cc128_get_uperms(result); }

bool cc128_is_sealed(cc128_cap_t *result) { return cc128_is_cap_sealed(result); }