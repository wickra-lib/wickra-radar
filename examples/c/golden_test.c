/* Cross-language golden parity and operating-mode equivalence, from C.
 *
 * Golden: build the radar from each committed golden/specs/*.json, scan the
 * shared golden/events.json and assert the report equals
 * golden/expected/<spec>.json byte-for-byte. The ABI returns the core's
 * compact command output verbatim, so byte equality is the exact
 * cross-language parity check -- the same one Python, Node, Go, C#, Java, R
 * and WASM make.
 *
 * Operating mode: the radar has two ways to consume a universe -- one-shot
 * `scan` over every symbol, or streaming `feed_batch` per symbol followed by
 * `alerts` -- and both must return the same bytes. The core pins this in Rust
 * (streaming_eq_batch.rs); this checks the boundary six of the ten language
 * reaches cross. C has no JSON library, so the per-symbol arrays are cut out
 * of the events text by bracket depth and fed as the exact bytes the file
 * carries.
 *
 * C has no directory API that is portable between POSIX and Windows, so the
 * spec list is globbed by CMake at configure time and written into
 * golden_specs.h. That keeps the property the other bindings get from a
 * runtime glob: a spec added to the corpus is covered here without editing
 * this file.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "golden_specs.h" /* GOLDEN_DIR, GOLDEN_SPECS, GOLDEN_SPEC_COUNT */
#include "wickra_radar.h"

/* Read a whole file. Caller frees. Returns NULL and reports on failure. */
static char *slurp(const char *path) {
    FILE *file = fopen(path, "rb");
    if (!file) {
        fprintf(stderr, "cannot open %s\n", path);
        return NULL;
    }
    if (fseek(file, 0, SEEK_END) != 0) {
        fclose(file);
        return NULL;
    }
    long size = ftell(file);
    if (size < 0 || fseek(file, 0, SEEK_SET) != 0) {
        fclose(file);
        return NULL;
    }
    char *buf = (char *)malloc((size_t)size + 1);
    if (!buf) {
        fclose(file);
        return NULL;
    }
    size_t got = fread(buf, 1, (size_t)size, file);
    fclose(file);
    buf[got] = '\0';
    return buf;
}

/* Trim ASCII whitespace in place and return the start of the trimmed text. */
static char *trim(char *text) {
    while (*text == ' ' || *text == '\n' || *text == '\r' || *text == '\t') {
        text++;
    }
    size_t len = strlen(text);
    while (len > 0) {
        char last = text[len - 1];
        if (last != ' ' && last != '\n' && last != '\r' && last != '\t') {
            break;
        }
        text[--len] = '\0';
    }
    return text;
}

/* A growable string. */
typedef struct {
    char *buf;
    size_t len;
    size_t cap;
} Str;

static int str_push(Str *s, const char *text, size_t n) {
    if (s->len + n + 1 > s->cap) {
        size_t cap = s->cap ? s->cap : 4096;
        while (cap < s->len + n + 1) {
            cap *= 2;
        }
        char *grown = (char *)realloc(s->buf, cap);
        if (!grown) {
            return 0;
        }
        s->buf = grown;
        s->cap = cap;
    }
    memcpy(s->buf + s->len, text, n);
    s->len += n;
    s->buf[s->len] = '\0';
    return 1;
}

static int str_puts(Str *s, const char *text) { return str_push(s, text, strlen(text)); }

/* Apply one read-only command through the two-call length protocol. Caller
 * frees. Every call executes the command, so this is only for commands that
 * leave the radar unchanged (scan, alerts); a stateful one goes through
 * run_once. */
static char *run(WickraRadar *radar, const char *cmd) {
    int32_t len = wickra_radar_command(radar, cmd, NULL, 0);
    if (len < 0) {
        fprintf(stderr, "command failed with code %d\n", (int)len);
        return NULL;
    }
    char *out = (char *)malloc((size_t)len + 1);
    if (!out) {
        return NULL;
    }
    if (wickra_radar_command(radar, cmd, out, (size_t)len + 1) < 0) {
        free(out);
        return NULL;
    }
    return out;
}

/* Apply one stateful command exactly once, into a buffer big enough for its
 * acknowledgement. feed_batch answers with a short status object, so a
 * length-then-fill round trip would feed the events twice. Returns 0 on
 * failure. */
static int run_once(WickraRadar *radar, const char *cmd) {
    char ack[4096];
    int32_t len = wickra_radar_command(radar, cmd, ack, sizeof ack);
    if (len < 0) {
        fprintf(stderr, "command failed with code %d\n", (int)len);
        return 0;
    }
    if ((size_t)len >= sizeof ack) {
        fprintf(stderr, "acknowledgement of %d bytes does not fit\n", (int)len);
        return 0;
    }
    if (strncmp(ack, "{\"ok\":true", 10) != 0) {
        fprintf(stderr, "command rejected: %s\n", ack);
        return 0;
    }
    return 1;
}

/* `{"cmd":"scan","events":<events>}` */
static char *scan(const char *spec, const char *events) {
    WickraRadar *radar = wickra_radar_new(spec);
    if (!radar) {
        fprintf(stderr, "invalid spec\n");
        return NULL;
    }
    Str cmd = {0};
    if (!str_puts(&cmd, "{\"cmd\":\"scan\",\"events\":") || !str_puts(&cmd, events) || !str_puts(&cmd, "}")) {
        wickra_radar_free(radar);
        return NULL;
    }
    char *out = run(radar, cmd.buf);
    free(cmd.buf);
    wickra_radar_free(radar);
    return out;
}

/* Walk `{"SYM":[...], ...}` and feed each symbol's array with feed_batch, then
 * read `alerts`. The events text is the file's own bytes, cut at bracket
 * depth; strings inside events never contain brackets in this corpus, and a
 * symbol name is a plain identifier. */
static char *stream_batched(const char *spec, const char *events) {
    WickraRadar *radar = wickra_radar_new(spec);
    if (!radar) {
        fprintf(stderr, "invalid spec\n");
        return NULL;
    }
    const char *p = events;
    int fed = 0;
    for (;;) {
        /* the next key: "..." followed by : and [ */
        const char *quote = strchr(p, '"');
        if (!quote) {
            break;
        }
        const char *end_quote = strchr(quote + 1, '"');
        if (!end_quote) {
            break;
        }
        const char *after = end_quote + 1;
        while (*after == ' ' || *after == '\n' || *after == '\r' || *after == '\t') {
            after++;
        }
        if (*after != ':') {
            p = end_quote + 1;
            continue;
        }
        after++;
        while (*after == ' ' || *after == '\n' || *after == '\r' || *after == '\t') {
            after++;
        }
        if (*after != '[') {
            p = end_quote + 1;
            continue;
        }
        const char *start = after;
        int depth = 0;
        const char *q = start;
        for (; *q; q++) {
            if (*q == '[') {
                depth++;
            } else if (*q == ']') {
                depth--;
                if (depth == 0) {
                    q++;
                    break;
                }
            }
        }
        Str cmd = {0};
        int ok = str_puts(&cmd, "{\"cmd\":\"feed_batch\",\"symbol\":") &&
                 str_push(&cmd, quote, (size_t)(end_quote + 1 - quote)) &&
                 str_puts(&cmd, ",\"events\":") &&
                 str_push(&cmd, start, (size_t)(q - start)) && str_puts(&cmd, "}");
        if (!ok) {
            free(cmd.buf);
            wickra_radar_free(radar);
            return NULL;
        }
        int accepted = run_once(radar, cmd.buf);
        free(cmd.buf);
        if (!accepted) {
            wickra_radar_free(radar);
            return NULL;
        }
        fed++;
        p = q;
    }
    if (fed == 0) {
        fprintf(stderr, "no symbols found in the events text\n");
        wickra_radar_free(radar);
        return NULL;
    }
    char *out = run(radar, "{\"cmd\":\"alerts\"}");
    wickra_radar_free(radar);
    return out;
}

int main(void) {
    printf("wickra-radar %s: golden parity + operating modes over %zu spec(s)\n",
           wickra_radar_version(), (size_t)GOLDEN_SPEC_COUNT);
    if (GOLDEN_SPEC_COUNT == 0) {
        fprintf(stderr, "golden corpus not found\n");
        return 1;
    }
    char *events = slurp(GOLDEN_DIR "/events.json");
    if (!events) {
        return 1;
    }
    int failures = 0;
    for (size_t i = 0; GOLDEN_SPECS[i]; i++) {
        char spec_path[1024];
        char expected_path[1024];
        snprintf(spec_path, sizeof spec_path, "%s/specs/%s", GOLDEN_DIR, GOLDEN_SPECS[i]);
        snprintf(expected_path, sizeof expected_path, "%s/expected/%s", GOLDEN_DIR, GOLDEN_SPECS[i]);
        char *spec = slurp(spec_path);
        char *expected_raw = slurp(expected_path);
        if (!spec || !expected_raw) {
            free(spec);
            free(expected_raw);
            failures++;
            continue;
        }
        char *expected = trim(expected_raw);
        char *batch = scan(spec, events);
        char *streamed = stream_batched(spec, events);
        if (!batch || !streamed) {
            fprintf(stderr, "%s: command failed\n", GOLDEN_SPECS[i]);
            failures++;
        } else if (strcmp(trim(batch), expected) != 0) {
            fprintf(stderr, "%s: scan does not match the blessed report\n", GOLDEN_SPECS[i]);
            failures++;
        } else if (strcmp(trim(streamed), trim(batch)) != 0) {
            const char *a = trim(streamed);
            const char *b = trim(batch);
            size_t at = 0;
            while (a[at] && b[at] && a[at] == b[at]) {
                at++;
            }
            fprintf(stderr, "%s: feed_batch + alerts differs from scan at byte %zu\n  streamed: %.80s\n  scan:     %.80s\n",
                    GOLDEN_SPECS[i], at, a + at, b + at);
            failures++;
        } else {
            printf("  %s: ok\n", GOLDEN_SPECS[i]);
        }
        free(batch);
        free(streamed);
        free(spec);
        free(expected_raw);
    }
    free(events);
    if (failures) {
        fprintf(stderr, "%d spec(s) failed\n", failures);
        return 1;
    }
    printf("ok\n");
    return 0;
}
