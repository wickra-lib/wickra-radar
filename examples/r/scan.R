# A runnable R example: scan a perp universe through the binding.
#
#   cargo build -p wickra-radar-c --release
#   export WKRADAR_LIB="$PWD/target/release"
#   export LD_LIBRARY_PATH="$WKRADAR_LIB:$LD_LIBRARY_PATH"   # PATH on Windows
#   R CMD INSTALL bindings/r
#   Rscript examples/r/scan.R

library(wickraradar)

spec <- paste0(
  '{"symbols":["AAA"],"signals":[',
  '{"kind":"funding_flip","params":[0.0005]}],"threshold":0.0}'
)

scan_cmd <- paste0(
  '{"cmd":"scan","events":{"AAA":[',
  '{"kind":"derivatives","ts":1,"open_interest":1.0,"funding_rate":0.0003,"mark_price":50.0},',
  '{"kind":"derivatives","ts":2,"open_interest":1.0,"funding_rate":-0.0004,"mark_price":50.0}]}}'
)

radar <- wkradar_new(spec)
response <- wkradar_command(radar, scan_cmd)

cat("wickra-radar", wkradar_version(), "\n")
cat(response, "\n")
