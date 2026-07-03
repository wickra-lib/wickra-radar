/* A minimal C example: scan a perp universe through the wickra-radar C ABI. */
#include <stdio.h>
#include <stdlib.h>

#include "wickra_radar.h"

static const char *SPEC =
    "{\"symbols\":[\"AAA\"],\"signals\":["
    "{\"kind\":\"funding_flip\",\"params\":[0.0005]}],\"threshold\":0.0}";

static const char *SCAN =
    "{\"cmd\":\"scan\",\"events\":{\"AAA\":["
    "{\"kind\":\"derivatives\",\"ts\":1,\"open_interest\":1.0,\"funding_rate\":0.0003,\"mark_price\":50.0},"
    "{\"kind\":\"derivatives\",\"ts\":2,\"open_interest\":1.0,\"funding_rate\":-0.0004,\"mark_price\":50.0}]}}";

/* Length-out protocol: learn the length, then read into a caller buffer.
   Returns a malloc'd NUL-terminated string the caller must free, or NULL. */
static char *run(WickraRadar *radar, const char *cmd) {
    int len = wickra_radar_command(radar, cmd, NULL, 0);
    if (len < 0) {
        fprintf(stderr, "command failed: code %d\n", len);
        return NULL;
    }
    char *buf = (char *)malloc((size_t)len + 1);
    if (!buf) {
        return NULL;
    }
    wickra_radar_command(radar, cmd, buf, (size_t)len + 1);
    return buf;
}

int main(void) {
    WickraRadar *radar = wickra_radar_new(SPEC);
    if (!radar) {
        fprintf(stderr, "failed to build radar\n");
        return 1;
    }

    char *report = run(radar, SCAN);
    if (!report) {
        wickra_radar_free(radar);
        return 1;
    }

    printf("wickra-radar %s\n", wickra_radar_version());
    printf("report: %s\n", report);

    free(report);
    wickra_radar_free(radar);
    return 0;
}
