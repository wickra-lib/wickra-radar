// A minimal C++ example: scan a perp universe through the wickra-radar C ABI.
#include <cstddef>
#include <iostream>
#include <string>
#include <vector>

#include "wickra_radar.h"

namespace {
const char *SPEC =
    R"({"symbols":["AAA"],"signals":[)"
    R"({"kind":"funding_flip","params":[0.0005]}],"threshold":0.0})";

const char *SCAN =
    R"({"cmd":"scan","events":{"AAA":[)"
    R"({"kind":"derivatives","ts":1,"open_interest":1.0,"funding_rate":0.0003,"mark_price":50.0},)"
    R"({"kind":"derivatives","ts":2,"open_interest":1.0,"funding_rate":-0.0004,"mark_price":50.0}]}})";

// Length-out protocol: learn the length, then read into a caller buffer.
std::string run(WickraRadar *radar, const char *cmd) {
    int len = wickra_radar_command(radar, cmd, nullptr, 0);
    if (len < 0) {
        std::cerr << "command failed: code " << len << "\n";
        return {};
    }
    std::vector<char> buf(static_cast<std::size_t>(len) + 1);
    wickra_radar_command(radar, cmd, buf.data(),
                         static_cast<std::size_t>(buf.size()));
    return std::string(buf.data());
}
}  // namespace

int main() {
    WickraRadar *radar = wickra_radar_new(SPEC);
    if (radar == nullptr) {
        std::cerr << "failed to build radar\n";
        return 1;
    }

    std::string report = run(radar, SCAN);

    std::cout << "wickra-radar " << wickra_radar_version() << "\n";
    std::cout << "report: " << report << "\n";

    wickra_radar_free(radar);
    return 0;
}
