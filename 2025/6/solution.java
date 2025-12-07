import java.io.IOException;
import java.util.stream.Stream;

final String INPUT_FILE = "./input.txt";

void main() throws IOException {
    String[][] lines = Files.lines(Path.of(INPUT_FILE))
        .map(line -> line.split("[ ]+"))
        .toArray(String[][]::new);

    // Transpose
    List<List<String>> transposed = IntStream.range(0, lines[0].length)
        .mapToObj(i -> Stream.of(lines).map(row -> row[i]).toList())
        .toList();
    
    // Solution 1
    System.out.println(
        transposed.stream()
            .mapToLong(row -> {
                LongStream values = row
                        .subList(0, row.size() - 1)
                        .stream()
                        .mapToLong(i -> Long.valueOf(i));
                return row.get(row.size() - 1).equals("+") ?
                        values.sum() :
                        values.reduce(1, (a, b) -> a * b);
            }).sum());

    // Solution 2
    System.out.println(
        transposed.stream()
            .mapToLong(column -> {
                return 10L;
            }).sum());
}