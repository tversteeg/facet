
#if defined(_WIN32) || defined(_WIN64)
    #define EXPORT __declspec(dllexport)
#elif defined(__GNUC__)
    #define EXPORT __attribute__((visibility("default")))
#else
    #define EXPORT
#endif

EXPORT const char* get_library_message(void) {
    return "IAMA C lib AMA";
}
