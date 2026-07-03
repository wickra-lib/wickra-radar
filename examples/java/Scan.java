// A runnable Java example: scan a perp universe through the binding.
//
//   cargo build -p wickra-radar-c
//   mvn -f bindings/java/pom.xml -q package -DskipTests
//   javac -cp bindings/java/target/classes examples/java/Scan.java -d examples/java/out
//   java --enable-native-access=ALL-UNNAMED \
//        -Dnative.lib.dir=target/debug \
//        -cp "bindings/java/target/classes;examples/java/out" Scan
import org.wickra.radar.Radar;

public final class Scan {
    private static final String SPEC =
            "{\"symbols\":[\"AAA\"],\"signals\":["
                    + "{\"kind\":\"funding_flip\",\"params\":[0.0005]}],\"threshold\":0.0}";

    private static final String SCAN =
            "{\"cmd\":\"scan\",\"events\":{\"AAA\":["
                    + "{\"kind\":\"derivatives\",\"ts\":1,\"open_interest\":1.0,\"funding_rate\":0.0003,\"mark_price\":50.0},"
                    + "{\"kind\":\"derivatives\",\"ts\":2,\"open_interest\":1.0,\"funding_rate\":-0.0004,\"mark_price\":50.0}]}}";

    public static void main(String[] args) {
        try (Radar radar = new Radar(SPEC)) {
            String response = radar.command(SCAN);
            System.out.println("wickra-radar " + Radar.version());
            System.out.println(response);
        }
    }
}
