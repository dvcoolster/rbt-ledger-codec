#pragma once
#include <stdint.h>
#ifdef __cplusplus
extern "C" {
#endif

// Identity ANS-X encode/decode (pass-through)
uint8_t* ansx_encode(const uint8_t* input, uint32_t len, uint32_t* out_len);
uint8_t* ansx_decode(const uint8_t* input, uint32_t len, uint32_t* out_len);
void ansx_free(void* ptr, uint32_t len);

#ifdef __cplusplus
}
#endif 