#ifndef sha256_h
#define sha256_h

#include <unistd.h>
void sha256(const unsigned char *data, size_t len, unsigned char *out);

#endif /* sha256_h */
