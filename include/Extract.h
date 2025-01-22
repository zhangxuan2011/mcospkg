#ifndef EXTRACT_H
    #define EXTRACT_H
#endif

#if defined (__linux__)
    int extractArchiveLinux(const char* fileName, const char* extractPath);
    #define extractArchive extractArchiveLinux
#else
    #error Unsupported system!
#endif
