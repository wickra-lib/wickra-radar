/* R .Call glue for the wickra-radar C ABI hub. */
#include <R.h>
#include <Rinternals.h>
#include <R_ext/Rdynload.h>
#include <stddef.h>
#include "wickra_radar.h"

/* --- handle lifetime ----------------------------------------------------- */

static void wkradar_finalize(SEXP ext) {
    WickraRadar *h = (WickraRadar *)R_ExternalPtrAddr(ext);
    if (h) {
        wickra_radar_free(h);
    }
    R_ClearExternalPtr(ext);
}

static WickraRadar *handle_of(SEXP ext) {
    WickraRadar *h = (WickraRadar *)R_ExternalPtrAddr(ext);
    if (!h) {
        Rf_error("wickra-radar: handle is closed");
    }
    return h;
}

/* --- exported .Call entries ---------------------------------------------- */

SEXP wkradar_version(void) {
    return Rf_mkString(wickra_radar_version());
}

SEXP wkradar_new(SEXP spec_json) {
    WickraRadar *h = wickra_radar_new(CHAR(STRING_ELT(spec_json, 0)));
    if (!h) {
        Rf_error("wickra-radar: invalid spec");
    }
    SEXP ext = PROTECT(R_MakeExternalPtr(h, R_NilValue, R_NilValue));
    R_RegisterCFinalizerEx(ext, wkradar_finalize, TRUE);
    UNPROTECT(1);
    return ext;
}

SEXP wkradar_command(SEXP ext, SEXP cmd_json) {
    WickraRadar *h = handle_of(ext);
    const char *cmd = CHAR(STRING_ELT(cmd_json, 0));

    /* Length-out protocol: learn the length, then read into a caller buffer.
       Domain errors come back in-band as {"ok":false,...} JSON, not a negative
       code; only unusable arguments / a caught panic return < 0. */
    int len = wickra_radar_command(h, cmd, NULL, 0);
    if (len < 0) {
        Rf_error("wickra-radar: command failed (code %d)", len);
    }
    char *buf = (char *)R_alloc((size_t)len + 1, 1);
    wickra_radar_command(h, cmd, buf, (size_t)len + 1);
    return Rf_mkString(buf);
}

/* --- registration -------------------------------------------------------- */

static const R_CallMethodDef CallEntries[] = {
    {"wkradar_version", (DL_FUNC)&wkradar_version, 0},
    {"wkradar_new", (DL_FUNC)&wkradar_new, 1},
    {"wkradar_command", (DL_FUNC)&wkradar_command, 2},
    {NULL, NULL, 0}};

void R_init_wickraradar(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);
}
