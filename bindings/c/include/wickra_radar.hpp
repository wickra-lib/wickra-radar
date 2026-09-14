// Wickra Radar — C++ wrapper over the C ABI.
//
// Header-only, C++17, no dependency beyond the standard library and
// `wickra_radar.h` beside it. Link the same `wickra_radar` library the C
// binding does.
//
// What it adds over calling the C functions directly is the handling nobody
// wants to write twice: the handle is owned and freed, the two-call length
// protocol behind `wickra_radar_command` is done for you, and a failure comes
// back as an exception rather than a negative integer a caller can ignore.
//
//     #include <wickra_radar.hpp>
//
//     wickra::Radar radar(R"({"signals":[{"kind":"funding_flip","params":[0.0005]}]})");
//     radar.command(R"({"cmd":"feed","symbol":"BTCUSDT","event":{...}})");
//     std::string alerts = radar.command(R"({"cmd":"alerts"})");
//
// The radar is data-driven, so this wrapper deliberately stops at strings:
// the spec and the report are JSON, and which JSON library a caller uses is
// their choice, not this header's.

#ifndef WICKRA_RADAR_HPP
#define WICKRA_RADAR_HPP

#include <cstddef>
#include <cstdint>
#include <stdexcept>
#include <string>
#include <utility>

#include "wickra_radar.h"

namespace wickra {

/// Thrown when the library rejects a spec or a command.
class RadarError : public std::runtime_error {
 public:
  explicit RadarError(const std::string& what) : std::runtime_error(what) {}
};

/// An owning handle to a radar built from a spec.
///
/// Move-only, because the underlying handle is a unique resource: copying it
/// would free the same pointer twice.
class Radar {
 public:
  /// Build a radar from a spec JSON string.
  ///
  /// Throws `RadarError` if the spec is not valid JSON or not a valid spec.
  explicit Radar(const std::string& spec_json)
      : handle_(wickra_radar_new(spec_json.c_str())) {
    if (handle_ == nullptr) {
      throw RadarError("wickra_radar_new rejected the spec");
    }
  }

  ~Radar() { wickra_radar_free(handle_); }

  Radar(const Radar&) = delete;
  Radar& operator=(const Radar&) = delete;

  Radar(Radar&& other) noexcept : handle_(other.handle_) {
    other.handle_ = nullptr;
  }

  Radar& operator=(Radar&& other) noexcept {
    if (this != &other) {
      wickra_radar_free(handle_);
      handle_ = other.handle_;
      other.handle_ = nullptr;
    }
    return *this;
  }

  /// Apply a command JSON and return the response JSON.
  ///
  /// The C entry point writes into a caller buffer and reports the length it
  /// needed, so this asks for the length first and then reads; the hub caches
  /// the response of a mutating command (`feed`, `feed_batch`) between the
  /// two calls, so it is executed once, not twice. A command the
  /// library understands but cannot carry out answers in band with
  /// `{"ok":false,"error":...}`; a negative return is a failure of the call
  /// itself and becomes an exception.
  std::string command(const std::string& cmd_json) {
    const std::int32_t needed =
        wickra_radar_command(handle_, cmd_json.c_str(), nullptr, 0);
    if (needed < 0) {
      throw RadarError("wickra_radar_command failed with code " +
                       std::to_string(needed));
    }

    std::string out(static_cast<std::size_t>(needed), '\0');
    // The C side writes a trailing NUL, so the buffer has to hold one more byte
    // than the response itself.
    const std::int32_t written = wickra_radar_command(
        handle_, cmd_json.c_str(), out.data(),
        static_cast<std::uintptr_t>(out.size()) + 1);
    if (written < 0) {
      throw RadarError("wickra_radar_command failed with code " +
                       std::to_string(written));
    }
    if (written != needed) {
      // The response changed length between the two calls, which cannot happen
      // for a handle only this thread is using. Saying so is better than
      // returning a string that is half of one answer and half of another.
      throw RadarError("wickra_radar_command length changed between calls");
    }
    return out;
  }

  /// The library version.
  static std::string version() { return std::string(wickra_radar_version()); }

 private:
  WickraRadar* handle_;
};

}  // namespace wickra

#endif  // WICKRA_RADAR_HPP
