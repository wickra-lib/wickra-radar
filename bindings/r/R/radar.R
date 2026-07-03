#' The wickra-radar library version.
#' @return A version string.
#' @export
wkradar_version <- function() {
  .Call(C_wkradar_version)
}

#' Build a radar from a spec JSON string.
#' @param spec_json A JSON spec string.
#' @return A `wickra_radar` handle (an external pointer).
#' @export
wkradar_new <- function(spec_json) {
  .Call(C_wkradar_new, spec_json)
}

#' Apply a command JSON and return the resulting response JSON.
#' @param radar A radar handle from [wkradar_new()].
#' @param cmd_json A command JSON string.
#' @return The response as a JSON string.
#' @export
wkradar_command <- function(radar, cmd_json) {
  .Call(C_wkradar_command, radar, cmd_json)
}
