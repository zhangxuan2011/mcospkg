#include <stdio.h>
#include "pmio.h"

void putError(const char* message){
    tColorRed(); // Text v
    textAttr_bold(); // Attributes
    putn("ERROR: ");
    textAttr_reset();
    putn(message);
}