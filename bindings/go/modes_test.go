package wickra

// Operating-mode equivalence through the binding: one-shot `scan` over the
// universe and streaming `feed` / `feed_batch` per symbol followed by `alerts`
// must return the same report bytes for every golden spec. The core pins this
// in Rust (streaming_eq_batch.rs); this checks the boundary the Go binding
// crosses. A missing corpus is a failure, not a skip.

import (
	"encoding/json"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"testing"
)

func command(t *testing.T, r *Radar, cmd map[string]any) string {
	t.Helper()
	b, err := json.Marshal(cmd)
	if err != nil {
		t.Fatal(err)
	}
	out, err := r.Command(string(b))
	if err != nil {
		t.Fatal(err)
	}
	return strings.TrimSpace(out)
}

func symbolsOf(events map[string]json.RawMessage) []string {
	keys := make([]string, 0, len(events))
	for k := range events {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	return keys
}

func TestStreamingEqualsBatch(t *testing.T) {
	g := goldenDir()
	if g == "" {
		t.Fatal("golden corpus not found")
	}
	raw, err := os.ReadFile(filepath.Join(g, "events.json"))
	if err != nil {
		t.Fatal(err)
	}
	var events map[string]json.RawMessage
	if err := json.Unmarshal(raw, &events); err != nil {
		t.Fatal(err)
	}
	specs, err := filepath.Glob(filepath.Join(g, "specs", "*.json"))
	if err != nil || len(specs) == 0 {
		t.Fatal("golden corpus not found")
	}
	for _, specPath := range specs {
		name := filepath.Base(specPath)
		spec, err := os.ReadFile(specPath)
		if err != nil {
			t.Fatal(err)
		}
		expected, err := os.ReadFile(filepath.Join(g, "expected", name))
		if err != nil {
			t.Fatal(err)
		}

		batchRadar, err := New(string(spec))
		if err != nil {
			t.Fatalf("%s: %v", name, err)
		}
		batch := command(t, batchRadar, map[string]any{"cmd": "scan", "events": json.RawMessage(raw)})
		batchRadar.Close()
		if batch != strings.TrimSpace(string(expected)) {
			t.Fatalf("%s: scan does not match the blessed report", name)
		}

		streamRadar, err := New(string(spec))
		if err != nil {
			t.Fatalf("%s: %v", name, err)
		}
		for _, symbol := range symbolsOf(events) {
			var list []json.RawMessage
			if err := json.Unmarshal(events[symbol], &list); err != nil {
				t.Fatal(err)
			}
			for _, event := range list {
				command(t, streamRadar, map[string]any{"cmd": "feed", "symbol": symbol, "event": event})
			}
		}
		streamed := command(t, streamRadar, map[string]any{"cmd": "alerts"})
		streamRadar.Close()
		if streamed != batch {
			t.Fatalf("%s: feed + alerts differs from scan", name)
		}

		batchedRadar, err := New(string(spec))
		if err != nil {
			t.Fatalf("%s: %v", name, err)
		}
		for _, symbol := range symbolsOf(events) {
			command(t, batchedRadar, map[string]any{"cmd": "feed_batch", "symbol": symbol, "events": events[symbol]})
		}
		batched := command(t, batchedRadar, map[string]any{"cmd": "alerts"})
		batchedRadar.Close()
		if batched != batch {
			t.Fatalf("%s: feed_batch + alerts differs from scan", name)
		}
	}
}
