using System.Text.Json;
using Wickra.Radar;
using Xunit;

namespace WickraRadar.Tests;

// Cross-language golden parity: build the radar from each committed
// golden/specs/*.json, run scan over the shared golden/events.json and read back
// the report, then assert it equals golden/expected/<spec>.json byte-for-byte. The
// binding returns the core's compact command_json string verbatim, so byte
// equality is the exact cross-language parity check. The fixtures arrive in a
// later phase; until then the test skips cleanly.
public class GoldenTests
{
    private static string? FindGolden()
    {
        string? dir = AppContext.BaseDirectory;
        for (int i = 0; i < 10 && dir is not null; i++)
        {
            string g = Path.Combine(dir, "golden");
            if (Directory.Exists(Path.Combine(g, "specs")))
            {
                return g;
            }
            dir = Path.GetDirectoryName(dir);
        }
        return null;
    }

    [Fact]
    public void GoldenReports_AreByteIdentical()
    {
        string? golden = FindGolden();
        if (golden is null)
        {
            return; // golden fixtures not present yet
        }

        string eventsJson = File.ReadAllText(Path.Combine(golden, "events.json"));
        using JsonDocument events = JsonDocument.Parse(eventsJson);

        foreach (string specPath in Directory.GetFiles(Path.Combine(golden!, "specs"), "*.json"))
        {
            string spec = File.ReadAllText(specPath);
            string name = Path.GetFileName(specPath);
            string expected = File.ReadAllText(Path.Combine(golden!, "expected", name)).TrimEnd();

            using var radar = new Radar(spec);
            string scan = JsonSerializer.Serialize(new { cmd = "scan", events = events.RootElement });
            string raw = radar.Command(scan);
            Assert.Equal(expected, raw.TrimEnd());
        }
    }
}
