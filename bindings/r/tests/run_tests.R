## Plain-R tests for the wickra-radar R binding (no testthat dependency).
## Mirrors the Rust/Python/Node/Go/C#/Java tests and doubles as the completeness
## guard: it exercises the full public surface (version + new + command).

library(wickraradar)

spec <- paste0(
  '{"symbols":["AAA"],"signals":[{"kind":"funding_flip",',
  '"params":[0.0005]}],"threshold":0.0}'
)

deriv <- function(ts, funding) {
  paste0(
    '{"kind":"derivatives","ts":', ts, ',"open_interest":1.0,',
    '"funding_rate":', funding, ',"mark_price":50.0}'
  )
}

## version
stopifnot(nzchar(wkradar_version()))

## scan roundtrip
radar <- wkradar_new(spec)
scan_cmd <- paste0(
  '{"cmd":"scan","events":{"AAA":[',
  deriv(1, "0.0003"), ',',
  deriv(2, "-0.0004"), ']}}'
)
raw <- wkradar_command(radar, scan_cmd)
stopifnot(grepl('"scanned":1', raw, fixed = TRUE))
stopifnot(grepl('"symbol":"AAA"', raw, fixed = TRUE))
## a funding flip clamps the severity to 1.0
stopifnot(grepl('"severity":1.0', raw, fixed = TRUE))

## invalid spec raises
stopifnot(inherits(try(wkradar_new("not json"), silent = TRUE), "try-error"))

## an unknown command is an in-band error, not a hard error
inband <- wkradar_command(radar, '{"cmd":"nope"}')
stopifnot(grepl('"ok":false', inband, fixed = TRUE))

## cross-language golden parity: build the radar from each committed
## golden/specs/*.json, run scan over the shared golden/events.json and read back
## the report, and assert the response equals golden/expected/<spec>.json
## byte-for-byte. The binding returns the core's compact command output verbatim,
## so byte equality is the exact cross-language parity check. The fixtures arrive
## in a later phase; until then the golden section is skipped.
golden_dir <- function() {
  d <- normalizePath(getwd(), mustWork = FALSE)
  for (i in seq_len(8)) {
    g <- file.path(d, "golden")
    if (dir.exists(file.path(g, "specs"))) {
      return(g)
    }
    d <- dirname(d)
  }
  NULL
}

g <- golden_dir()
if (!is.null(g)) {
  events <- trimws(paste(
    readLines(file.path(g, "events.json"), warn = FALSE), collapse = "\n"
  ))
  scan_all <- paste0('{"cmd":"scan","events":', events, '}')
  for (spec_path in list.files(file.path(g, "specs"), pattern = "\\.json$", full.names = TRUE)) {
    name <- basename(spec_path)
    spec_json <- paste(readLines(spec_path, warn = FALSE), collapse = "\n")
    expected <- trimws(paste(
      readLines(file.path(g, "expected", name), warn = FALSE), collapse = "\n"
    ))
    gradar <- wkradar_new(spec_json)
    got <- wkradar_command(gradar, scan_all)
    stopifnot(identical(trimws(got), expected))
  }
}

cat("wickra-radar R tests passed\n")
