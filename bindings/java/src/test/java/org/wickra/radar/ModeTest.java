package org.wickra.radar;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Stream;
import org.junit.jupiter.api.Test;

// Operating-mode equivalence through the binding: one-shot `scan` over the
// universe and streaming `feed_batch` per symbol followed by `alerts` must
// return the same report bytes for every golden spec. The core pins this in
// Rust (streaming_eq_batch.rs); this checks the boundary the JVM binding
// crosses. A missing corpus is a failure, not a skip.
//
// The events file is `{"SYMBOL":[event, ...], ...}`; without a JSON library the
// per-symbol arrays are cut out of the text by bracket depth, and fed as the
// exact bytes the file carries.
class ModeTest {
    private static Path findGolden() {
        Path dir = Path.of("").toAbsolutePath();
        for (int i = 0; i < 8 && dir != null; i++) {
            Path g = dir.resolve("golden");
            if (Files.isDirectory(g.resolve("specs"))) {
                return g;
            }
            dir = dir.getParent();
        }
        return null;
    }

    /** Split `{"SYM":[...], ...}` into (symbol, array-text) pairs. */
    private static List<String[]> perSymbol(String events) {
        java.util.ArrayList<String[]> out = new java.util.ArrayList<>();
        Pattern key = Pattern.compile("\"([^\"]+)\"\\s*:\\s*\\[");
        int i = 0;
        while (true) {
            Matcher m = key.matcher(events);
            if (!m.find(i)) {
                break;
            }
            int start = m.end() - 1;
            int depth = 0;
            int end = start;
            for (; end < events.length(); end++) {
                char c = events.charAt(end);
                if (c == '[') depth++;
                if (c == ']') { depth--; if (depth == 0) { end++; break; } }
            }
            out.add(new String[] {m.group(1), events.substring(start, end)});
            i = end;
        }
        return out;
    }

    @Test
    void streamingEqualsBatchForEveryGoldenSpec() throws IOException {
        Path golden = findGolden();
        assertNotNull(golden, "golden corpus not found");
        String events = Files.readString(golden.resolve("events.json")).strip();
        List<String[]> symbols = perSymbol(events);
        assertFalse(symbols.isEmpty(), "no symbols in golden/events.json");
        String scan = "{\"cmd\":\"scan\",\"events\":" + events + "}";
        try (Stream<Path> specs = Files.list(golden.resolve("specs"))) {
            List<Path> list = specs.filter(p -> p.toString().endsWith(".json")).toList();
            assertFalse(list.isEmpty(), "golden corpus not found");
            for (Path specPath : list) {
                String spec = Files.readString(specPath);
                String name = specPath.getFileName().toString();
                String expected = Files.readString(golden.resolve("expected").resolve(name)).strip();
                String batch;
                try (Radar radar = new Radar(spec)) {
                    batch = radar.command(scan).strip();
                }
                assertEquals(expected, batch, name + ": scan does not match the blessed report");
                try (Radar radar = new Radar(spec)) {
                    for (String[] s : symbols) {
                        radar.command("{\"cmd\":\"feed_batch\",\"symbol\":\"" + s[0] + "\",\"events\":" + s[1] + "}");
                    }
                    assertEquals(batch, radar.command("{\"cmd\":\"alerts\"}").strip(),
                            name + ": feed_batch + alerts differs from scan");
                }
            }
        }
    }
}
