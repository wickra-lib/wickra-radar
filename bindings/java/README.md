# Wickra Radar — Java

JVM bindings for the `wickra-radar` data-driven core over its C ABI hub
(FFM / Panama, `java.lang.foreign`). Build a `Radar` from a spec JSON, drive it
with command JSON, read back the report — the same protocol as every other
binding.

## Requirements

- Java 22+ (the Foreign Function & Memory API is stable since 22).
- Run with `--enable-native-access=ALL-UNNAMED`.
- The native library (`wickra_radar`) must be resolvable — either on the library
  path or via the `native.lib.dir` system property pointing at the directory
  that holds `libwickra_radar.{so,dylib}` / `wickra_radar.dll`.

## Usage

```java
import org.wickra.radar.Radar;

String spec = """
    {"symbols":["AAA"],"signals":[{"kind":"funding_flip","params":[0.0005]}],"threshold":0.0}""";

try (Radar radar = new Radar(spec)) {
    String report = radar.command("""
        {"cmd":"scan","events":{"AAA":[
        {"kind":"derivatives","ts":1,"open_interest":1.0,"funding_rate":0.0003,"mark_price":50.0},
        {"kind":"derivatives","ts":2,"open_interest":1.0,"funding_rate":-0.0004,"mark_price":50.0}]}}""");
    System.out.println(report);
}
System.out.println(Radar.version());
```

## API

| Member | Description |
|--------|-------------|
| `new Radar(String specJson)` | Build a radar from a spec JSON (throws `IllegalArgumentException` on an invalid spec). |
| `String command(String cmdJson)` | Apply a command JSON, return the response JSON. |
| `static String version()` | The library version. |
| `close()` | Free the native handle (via `AutoCloseable`). |

Domain errors (a bad spec, an unknown command) come back in-band as
`{"ok":false,"error":...}` JSON, not as an exception.

## License

`MIT OR Apache-2.0`.
