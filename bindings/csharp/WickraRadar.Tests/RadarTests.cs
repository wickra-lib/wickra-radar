using System.Text.Json;
using Wickra.Radar;
using Xunit;

namespace WickraRadar.Tests;

public class RadarTests
{
    private const string Spec =
        "{\"symbols\":[\"AAA\"],\"signals\":[{\"kind\":\"funding_flip\"," +
        "\"params\":[0.0005]}],\"threshold\":0.0}";

    private static object Deriv(long ts, double oi, double funding, double mark) =>
        new { kind = "derivatives", ts, open_interest = oi, funding_rate = funding, mark_price = mark };

    [Fact]
    public void Version_IsNonEmpty()
    {
        Assert.False(string.IsNullOrEmpty(Radar.Version()));
    }

    [Fact]
    public void Scan_Roundtrip()
    {
        using var radar = new Radar(Spec);
        string scan = JsonSerializer.Serialize(new
        {
            cmd = "scan",
            events = new Dictionary<string, object[]>
            {
                ["AAA"] = new[]
                {
                    Deriv(1, 1.0, 0.0003, 50.0),
                    Deriv(2, 1.0, -0.0004, 50.0),
                },
            },
        });

        string raw = radar.Command(scan);
        using JsonDocument report = JsonDocument.Parse(raw);

        Assert.Equal(1, report.RootElement.GetProperty("scanned").GetInt32());
        JsonElement alerts = report.RootElement.GetProperty("alerts");
        Assert.Equal(1, alerts.GetArrayLength());
        Assert.Equal("AAA", alerts[0].GetProperty("symbol").GetString());
        // A funding flip clamps the severity to 1.0.
        Assert.Equal(1.0, alerts[0].GetProperty("severity").GetDouble(), 9);
    }

    [Fact]
    public void InvalidSpec_Throws()
    {
        Assert.Throws<ArgumentException>(() => new Radar("not json"));
    }

    [Fact]
    public void UnknownCommand_IsInBandError()
    {
        using var radar = new Radar(Spec);
        // An unknown command is not a hard error: the ABI returns a length and the
        // error surfaces in-band as {"ok":false,...} JSON.
        string raw = radar.Command("{\"cmd\":\"nope\"}");
        Assert.Contains("\"ok\":false", raw);
    }
}
