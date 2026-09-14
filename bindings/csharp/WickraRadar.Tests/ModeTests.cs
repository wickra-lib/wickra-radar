using System.Text.Json;
using Wickra.Radar;
using Xunit;

namespace WickraRadar.Tests;

// Operating-mode equivalence through the binding: one-shot `scan` over the
// universe and streaming `feed` / `feed_batch` per symbol followed by `alerts`
// must return the same report bytes for every golden spec. The core pins this
// in Rust (streaming_eq_batch.rs); this checks the boundary the .NET binding
// crosses. A missing corpus is a failure, not a skip.
public class ModeTests
{
    private static string FindGolden()
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
        throw new InvalidOperationException("golden corpus not found");
    }

    private static string Scan(string spec, JsonElement events)
    {
        using var radar = new Radar(spec);
        return radar.Command(JsonSerializer.Serialize(new { cmd = "scan", events })).Trim();
    }

    private static string Stream(string spec, JsonElement events)
    {
        using var radar = new Radar(spec);
        foreach (JsonProperty symbol in events.EnumerateObject())
        {
            foreach (JsonElement e in symbol.Value.EnumerateArray())
            {
                radar.Command(JsonSerializer.Serialize(new { cmd = "feed", symbol = symbol.Name, @event = e }));
            }
        }
        return radar.Command("{\"cmd\":\"alerts\"}").Trim();
    }

    private static string StreamBatched(string spec, JsonElement events)
    {
        using var radar = new Radar(spec);
        foreach (JsonProperty symbol in events.EnumerateObject())
        {
            radar.Command(JsonSerializer.Serialize(new { cmd = "feed_batch", symbol = symbol.Name, events = symbol.Value }));
        }
        return radar.Command("{\"cmd\":\"alerts\"}").Trim();
    }

    [Fact]
    public void StreamingEqualsBatch_ForEveryGoldenSpec()
    {
        string golden = FindGolden();
        using JsonDocument events = JsonDocument.Parse(File.ReadAllText(Path.Combine(golden, "events.json")));
        string[] specs = Directory.GetFiles(Path.Combine(golden, "specs"), "*.json");
        Assert.NotEmpty(specs);
        foreach (string specPath in specs)
        {
            string spec = File.ReadAllText(specPath);
            string name = Path.GetFileName(specPath);
            string expected = File.ReadAllText(Path.Combine(golden, "expected", name)).Trim();
            string batch = Scan(spec, events.RootElement);
            Assert.True(expected == batch, $"{name}: scan does not match the blessed report");
            Assert.True(Stream(spec, events.RootElement) == batch, $"{name}: feed + alerts differs from scan");
            Assert.True(StreamBatched(spec, events.RootElement) == batch, $"{name}: feed_batch + alerts differs from scan");
        }
    }
}
