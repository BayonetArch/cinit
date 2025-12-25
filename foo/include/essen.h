#ifndef ESSENTIAL_H
#define ESSENTIAL_H

#include <errno.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

#ifdef ESL_SHORT_NAMES

#define LogLevel Esl_LogLevel;
#define LOG_INFO ESL_LOG_INFO
#define LOG_WARN ESL_LOG_WARN
#define LOG_ERROR ESL_LOG_ERROR
#define UNUSED ESL_UNUSED
#define LAST_ERROR ESL_LAST_ERROR
#define SWAP ESL_SWAP)
#define current_date esl_current_date
#define eprintln esl_eprintln
#define println esl_println
#define fatalf esl_fatalf
#define flog esl_flog
#define fflog esl_fflog
#define shift_args esl_shift_args

#endif // ESL_SHORT_NAMES

typedef enum : int {
    ESL_LOG_INFO  = 0,
    ESL_LOG_WARN  = 1,
    ESL_LOG_ERROR = 2
} Esl_LogLevel;

#define ARR_LEN(arr) (sizeof(arr) / sizeof(arr[0]))
#define ESL_LAST_ERROR() strerror(errno)
#define FILE_LOC() __FILE__, __LINE__
#define SEPARATOR() println("───────────────────────────────────────")

#define ESL_SWAP(T, a, b)                                                      \
    do {                                                                       \
        T t = a;                                                               \
        a   = b;                                                               \
        b   = t;                                                               \
    } while (0)

#define ESL_UNUSED(x) (void)x

#include <time.h>

__attribute__((unused)) static inline char *esl_current_date(char  *time_buf,
                                                             size_t buf_size)
{
    time_t    t    = time(NULL);
    struct tm date = *localtime(&t);
    strftime(time_buf, buf_size, "%Y-%m-%d %I:%M:%S", &date);
    return time_buf;
}

/* Basic colors */
// clang-format off
#define ANSI_RESET "\x1b[0m"

#define ANSI_RED(x)                 "\x1b[0;31m" x ANSI_RESET
#define ANSI_RED_BOLD(x)            "\x1b[1;31m" x ANSI_RESET
#define ANSI_GREEN(x)               "\x1b[0;32m" x ANSI_RESET
#define ANSI_GREEN_BOLD(x)          "\x1b[1;32m" x ANSI_RESET
#define ANSI_GREY(x)                "\x1b[0;90m" x ANSI_RESET
#define ANSI_GREY_BOLD(x)           "\x1b[1;90m" x ANSI_RESET
#define ANSI_YELLOW(x)              "\x1b[0;33m" x ANSI_RESET
#define ANSI_YELLOW_BOLD(x)         "\x1b[1;33m" x ANSI_RESET
// clang-format on

#define esl_println(fmt, ...) printf(fmt "\n", ##__VA_ARGS__)
#define esl_eprintln(fmt, ...) fprintf(stderr, fmt "\n", ##__VA_ARGS__)

#define esl_fatalf(exit_code, fmt, ...)                                        \
    do {                                                                       \
        fprintf(stderr,                                                        \
                ANSI_GREY("%s:%d") " " ANSI_RED("Fatal") ": " fmt "\n",        \
                FILE_LOC(), ##__VA_ARGS__);                                    \
        exit(exit_code);                                                       \
    } while (0)

// file logging
#define esl_flog(fp, log_level, color, fmt, ...)                               \
    do {                                                                       \
        char time_buf[64];                                                     \
        current_date(time_buf, sizeof(time_buf));                              \
        switch ((log_level)) {                                                 \
        case ESL_LOG_WARN:                                                     \
            if (color) {                                                       \
                fprintf(fp,                                                    \
                        "%s "                                                  \
                        "[" ANSI_YELLOW("WRN") "] " fmt "\n",                \
                        time_buf, ##__VA_ARGS__);                              \
            }                                                                  \
            else {                                                             \
                fprintf(fp,                                                    \
                        "%s "                                                  \
                        "[WRN] " fmt "\n",                                   \
                        time_buf, ##__VA_ARGS__);                              \
            }                                                                  \
            break;                                                             \
        case ESL_LOG_INFO:                                                     \
            if (color) {                                                       \
                fprintf(fp,                                                    \
                        "%s "                                                  \
                        "[" ANSI_GREY("INF") "] " fmt "\n",                  \
                        time_buf, ##__VA_ARGS__);                              \
            }                                                                  \
            else {                                                             \
                fprintf(fp,                                                    \
                        "%s "                                                  \
                        "[INF] " fmt "\n",                                   \
                        time_buf, ##__VA_ARGS__);                              \
            }                                                                  \
            break;                                                             \
        case ESL_LOG_ERROR:                                                    \
            if (color) {                                                       \
                fprintf(fp,                                                    \
                        "%s "                                                  \
                        "[" ANSI_RED("ERR") "] " fmt "\n",                   \
                        time_buf, ##__VA_ARGS__);                              \
            }                                                                  \
            else {                                                             \
                fprintf(fp,                                                    \
                        "%s "                                                  \
                        "[ERR] " fmt "\n",                                   \
                        time_buf, ##__VA_ARGS__);                              \
            }                                                                  \
            break;                                                             \
        default:                                                               \
            esl_fatalf(1, "Not a valid log level");                            \
        };                                                                     \
    } while (0)

#define esl_fflog(fp1, fp2, log_level, color_for_f1, color_for_f2, fmt, ...)   \
    do {                                                                       \
        esl_flog(fp1, log_level, color_for_f1, fmt, ##__VA_ARGS__);            \
        esl_flog(fp2, log_level, color_for_f2, fmt, ##__VA_ARGS__);            \
    } while (0)

__attribute__((unused)) static inline const char *esl_shift_args(int   *argc,
                                                                 char **argv)
{
    if (*argc == 1)
        return NULL;

    const char *program = argv[0];
    for (int i = 0; i < *argc - 1; ++i) {
        argv[i] = argv[i + 1];
    }

    (*argc)--;
    return program;
}
#ifdef ESL_DA

#define ESL_DA_DEF_CAP 256

#define esl_da_append(xs, x)                                                   \
    do {                                                                       \
        if ((xs)->capacity == 0) {                                             \
            (xs)->capacity = ESL_DA_DEF_CAP;                                   \
            (xs)->items    = malloc((xs)->capacity * sizeof(*(xs)->items));    \
            if (!(xs)->items)                                                  \
                esl_fatalf(1, "memory allocation failed");                     \
        }                                                                      \
        else if ((xs)->count >= (xs)->capacity) {                              \
            (xs)->capacity *= 2;                                               \
            void *tmp =                                                        \
                realloc((xs)->items, (xs)->capacity * sizeof(*(xs)->items));   \
            if (!tmp)                                                          \
                esl_fatalf(1, "memory reallocation failed");                   \
            (xs)->items = tmp;                                                 \
        }                                                                      \
        (xs)->items[(xs)->count++] = x;                                        \
    } while (0)

#define esl_da_free(xs)                                                        \
    do {                                                                       \
        free((xs)->items);                                                     \
        ((xs))->items    = NULL;                                               \
        ((xs))->capacity = 0;                                                  \
        ((xs))->count    = 0;                                                  \
    } while (0)

#define esl_da_foreach(Type, it, da)                                           \
    for (Type *it = (da)->items; it < (da)->items + (da)->count; ++it)

#define esl_da_free_heap(type, xs)                                             \
    do {                                                                       \
        da_foreach(type, it, xs) { free(*it); }                                \
        da_free((xs));                                                         \
        (xs)->items    = NULL;                                                 \
        (xs)->capacity = 0;                                                    \
        (xs)->count    = 0;                                                    \
    } while (0);

#ifdef ESL_SHORT_NAMES

#define da_append(xs, x) esl_da_append(xs, x)
#define da_free esl_da_free
#define da_foreach(Type, it, da) esl_da_foreach(Type, it, da)
#define da_free_heap(type, xs) esl_da_free_heap(type, xs)

#endif /* ifdef ESL_SHORT_NAMES */

#endif // ESL_DA

#ifdef ESL_SB

#define ESL_DEF_SB_CAP 64
#include <stddef.h>

typedef struct {
    char  *buf;
    size_t size;
    size_t capacity;

} EslStringBuilder;

static inline void esl_sb_init(EslStringBuilder *sb)
{
    sb->capacity = ESL_DEF_SB_CAP;
    sb->buf      = malloc(sb->capacity);
    if (!sb->buf) {
        esl_fatalf(1, "memory allocation failed");
    }
    sb->size = 0;
}

static inline void esl_sb_append(EslStringBuilder *sb, char *str)
{
    size_t n = strlen(str);

    while (sb->size + n > sb->capacity) {
        sb->capacity *= 2;
    }
    char *tmp = realloc(sb->buf, sb->capacity);
    if (!tmp) {
        esl_fatalf(1, "Reallocation failed");
    }
    sb->buf = tmp;
    memcpy(sb->buf + sb->size, str, n);
    sb->size += n;
}

static inline char *esl_sb_to_string(EslStringBuilder *sb)
{
    sb->buf[sb->size] = '\0';

    return sb->buf;
}

static inline void esl_sb_free(EslStringBuilder *sb)
{
    free(sb->buf);
    sb->buf      = NULL;
    sb->capacity = 0;
    sb->size     = 0;
}

#ifdef ESL_SHORT_NAMES

#define sb_init esl_sb_init
#define sb_append esl_sb_append
#define sb_to_string esl_sb_to_string
#define sb_free esl_sb_free
typedef EslStringBuilder StringBuilder;

#endif /* ifdef ESL_SHORT_NAMES */

#endif /* ifdef ESL_SB */
#endif // ESSENTIAL_H
