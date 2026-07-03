// A runnable .NET example: scan a perp universe through the binding.
//
//   cargo build --release -p wickra-radar-c
//   dotnet run --project examples/csharp/Scan

using System.Text.Json;
using Wickra.Radar;

const string spec =
    "{\"symbols\":[\"AAA\"],\"signals\":[" +
    "{\"kind\":\"funding_flip\",\"params\":[0.0005]}],\"threshold\":0.0}";

const string scan =
    "{\"cmd\":\"scan\",\"events\":{\"AAA\":[" +
    "{\"kind\":\"derivatives\",\"ts\":1,\"open_interest\":1.0,\"funding_rate\":0.0003,\"mark_price\":50.0}," +
    "{\"kind\":\"derivatives\",\"ts\":2,\"open_interest\":1.0,\"funding_rate\":-0.0004,\"mark_price\":50.0}]}}";

using var radar = new Radar(spec);
string response = radar.Command(scan);
using JsonDocument report = JsonDocument.Parse(response);

Console.WriteLine($"wickra-radar {Radar.Version()}");
Console.WriteLine(response);
Console.WriteLine($"  alerts: {report.RootElement.GetProperty("alerts").GetArrayLength()}");
