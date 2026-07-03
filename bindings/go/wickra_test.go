package wickra

import (
	"encoding/json"
	"math"
	"strings"
	"testing"
)

const spec = `{"symbols":["AAA"],"signals":[{"kind":"funding_flip","params":[0.0005]}],"threshold":0.0}`

func deriv(ts int, oi, funding, mark float64) map[string]any {
	return map[string]any{
		"kind": "derivatives", "ts": ts,
		"open_interest": oi, "funding_rate": funding, "mark_price": mark,
	}
}

func TestVersion(t *testing.T) {
	if Version() == "" {
		t.Fatal("empty version")
	}
}

func TestScanRoundtrip(t *testing.T) {
	r, err := New(spec)
	if err != nil {
		t.Fatal(err)
	}
	defer r.Close()

	events := map[string]any{"AAA": []map[string]any{
		deriv(1, 1.0, 0.0003, 50.0),
		deriv(2, 1.0, -0.0004, 50.0),
	}}
	scan, err := json.Marshal(map[string]any{"cmd": "scan", "events": events})
	if err != nil {
		t.Fatal(err)
	}

	raw, err := r.Command(string(scan))
	if err != nil {
		t.Fatal(err)
	}
	var report struct {
		Scanned int `json:"scanned"`
		Alerts  []struct {
			Symbol   string  `json:"symbol"`
			Severity float64 `json:"severity"`
		} `json:"alerts"`
	}
	if err := json.Unmarshal([]byte(raw), &report); err != nil {
		t.Fatal(err)
	}
	if report.Scanned != 1 {
		t.Fatalf("expected scanned 1, got %d", report.Scanned)
	}
	if len(report.Alerts) != 1 || report.Alerts[0].Symbol != "AAA" {
		t.Fatalf("expected one AAA alert, got %+v", report.Alerts)
	}
	// A funding flip clamps the severity to 1.0.
	if math.Abs(report.Alerts[0].Severity-1.0) > 1e-9 {
		t.Fatalf("expected severity 1.0, got %v", report.Alerts[0].Severity)
	}
}

func TestInvalidSpec(t *testing.T) {
	if _, err := New("not json"); err == nil {
		t.Fatal("expected an error for an invalid spec")
	}
}

func TestUnknownCommandIsInBandError(t *testing.T) {
	r, err := New(spec)
	if err != nil {
		t.Fatal(err)
	}
	defer r.Close()

	// An unknown command is not a hard error: the C ABI returns a length and the
	// error surfaces in-band as {"ok":false,...} JSON.
	raw, err := r.Command(`{"cmd":"nope"}`)
	if err != nil {
		t.Fatalf("unexpected hard error: %v", err)
	}
	if !strings.Contains(raw, `"ok":false`) {
		t.Fatalf("expected an in-band error, got: %s", raw)
	}
}
