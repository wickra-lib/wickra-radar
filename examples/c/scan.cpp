// A minimal C++ example: scan a perp universe, then feed the same events one
// at a time and read the alerts back -- both through the C++ hull.
//
// This goes through `wickra_radar.hpp`, the C++ hull shipped beside the C
// header, because that hull is what a C++ caller is meant to use: it owns and
// frees the handle, runs the two-call length protocol behind
// `wickra_radar_command` for you -- the core carries the produced-but-
// undelivered response between the two calls, so a mutating `feed` runs once,
// not twice -- and turns a refusal into an exception rather than a negative
// integer that is easy to ignore. Calling the C functions directly from C++
// works too, but then the hull would be shipped without anything building it.
#include <cstdio>
#include <string>

#include "wickra_radar.hpp"

namespace {
const char *SPEC =
    R"({"symbols":["AAA"],"signals":[)"
    R"({"kind":"funding_flip","params":[0.0005]}],"threshold":0.0})";

const char *EVENT_1 =
    R"({"kind":"derivatives","ts":1,"open_interest":1.0,"funding_rate":0.0003,"mark_price":50.0})";
const char *EVENT_2 =
    R"({"kind":"derivatives","ts":2,"open_interest":1.0,"funding_rate":-0.0004,"mark_price":50.0})";
}  // namespace

int main() {
    try {
        std::printf("wickra-radar %s\n", wickra::Radar::version().c_str());

        // One-shot: the whole universe in a single `scan`.
        wickra::Radar batch(SPEC);
        const std::string scanned = batch.command(
            std::string(R"({"cmd":"scan","events":{"AAA":[)") + EVENT_1 + "," + EVENT_2 + "]}}");
        std::printf("report: %s\n", scanned.c_str());

        // Streaming: the same events fed as they arrive, then `alerts`.
        wickra::Radar live(SPEC);
        live.command(std::string(R"({"cmd":"feed","symbol":"AAA","event":)") + EVENT_1 + "}");
        live.command(std::string(R"({"cmd":"feed","symbol":"AAA","event":)") + EVENT_2 + "}");
        const std::string alerts = live.command(R"({"cmd":"alerts"})");
        std::printf("alerts: %s\n", alerts.c_str());

        // Both operating modes return the same report bytes.
        if (alerts != scanned) {
            std::fprintf(stderr, "streaming and batch disagree\n");
            return 1;
        }
    } catch (const wickra::RadarError &err) {
        // Every failure arrives here: a spec the core rejects, a command it does
        // not understand, a call that returned a negative code.
        std::fprintf(stderr, "%s\n", err.what());
        return 1;
    }
    return 0;
}
