#include <stdio.h>
#include "TextAttributes.h"
void _flush(){ //刷新缓冲区
    fflush(stdout);
    fflush(stderr);
}
void tColorBlack(){
    printf("\e[0;30m");
    _flush();
}
void tColorRed(){
    printf("\033[31m");
    _flush();
}
void tColorGreen(){
    printf("\e[0;32m");
    _flush();
}
void tColorBrown(){
    printf("\e[0;33m");
    _flush();
}
void tColorYellow(){
    printf("\e[1;33m");
    _flush();
}
void tColorBlue(){
    printf("\e[0;34m");
    _flush();
}
void tColorPurple(){
    printf("\e[0;35m");
    _flush();
}
void tColorCyan(){
    printf("\e[0;36m");
    _flush();
}
void tColorGray(){
    printf("\e[0;37m");
    _flush();
}
void tColorWhite(){
    printf("\e[1;37m");
    _flush();
}
void tColorLightBlack(){
    printf("\e[1;30m");
    _flush();
}
void tColorLightRed(){
    printf("\e[1;31m");
    _flush();
}
void tColorLightGreen(){
    printf("\e[1;32m");
    _flush();
}
void tColorLightBlue(){
    printf("\e[1;34m");
    _flush();
}
void tColorLightPurple(){
    printf("\e[1;35m");
    _flush();
}
void tColorLightCyan(){
    printf("\e[1;36m");
    _flush();
}
void textattr_bold(){
    printf("\e[1m");
    _flush();
}
void textattr_underline(){
    printf("\e[4m");
    _flush();
}
void textattr_Blink(){
    printf("\e[5m");
    _flush();
}
void textattr_hide(){
    printf("\e[8m");
    _flush();
}
void textattr_clear(){
    printf("\e[2J");
    _flush();
}
void textAttr_reset(){ // reset text attributes
    printf("\033[0m");
    _flush();
}