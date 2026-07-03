package org.wickra.radar;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.Test;

class RadarTest {
    private static final String SPEC =
            "{\"symbols\":[\"AAA\"],\"signals\":[{\"kind\":\"funding_flip\","
                    + "\"params\":[0.0005]}],\"threshold\":0.0}";

    private static String deriv(int ts, String funding) {
        return "{\"kind\":\"derivatives\",\"ts\":" + ts + ",\"open_interest\":1.0,"
                + "\"funding_rate\":" + funding + ",\"mark_price\":50.0}";
    }

    @Test
    void versionIsNonEmpty() {
        assertFalse(Radar.version().isEmpty());
    }

    @Test
    void scanRoundtrip() {
        try (Radar radar = new Radar(SPEC)) {
            String scan = "{\"cmd\":\"scan\",\"events\":{\"AAA\":["
                    + deriv(1, "0.0003") + ","
                    + deriv(2, "-0.0004") + "]}}";
            String raw = radar.command(scan);
            assertTrue(raw.contains("\"scanned\":1"), raw);
            assertTrue(raw.contains("\"symbol\":\"AAA\""), raw);
            // A funding flip clamps the severity to 1.0.
            assertTrue(raw.contains("\"severity\":1.0"), raw);
        }
    }

    @Test
    void invalidSpecThrows() {
        assertThrows(IllegalArgumentException.class, () -> new Radar("not json"));
    }

    @Test
    void unknownCommandIsInBandError() {
        try (Radar radar = new Radar(SPEC)) {
            // An unknown command is not a hard error: the ABI returns a length and
            // the error surfaces in-band as {"ok":false,...} JSON.
            String raw = radar.command("{\"cmd\":\"nope\"}");
            assertEquals(true, raw.contains("\"ok\":false"), raw);
        }
    }
}
