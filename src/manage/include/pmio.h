#ifndef PMIO_H
    #define PMIO_H
#endif

#include <stdio.h>
#include "TextAttributes.h"

inline void putn(const char* str){
    printf("%s",str);
}
inline void putn(char* str){
    printf("%s",str);
}
inline void putn(char chr){
    printf("%c",chr);
}
inline void putn(int num){
    printf("%d",num);
}
inline void putn(float num){
    printf("%f",num);
}
inline void putn(double num){
    printf("%f",num);
}
inline void putn(long unsigned int num){
    printf("%lu",num);
}
inline void putn(long long unsigned int num){
    printf("%llu",num);
}
void putError(const char* message);