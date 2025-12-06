import java.io.IOException;
import java.util.stream.IntStream;

final String INPUT_FILE = "./input.txt";
final int LENGTH = 12;
int startIndex;

void main() throws IOException {
    System.out.println(
        Files.lines(Path.of(INPUT_FILE))
            .mapToLong((bank) -> {
                startIndex = 0;
                return Long.valueOf(
                    IntStream.range(0, LENGTH).map((i) -> {
                        String substring = bank.substring(startIndex, bank.length() - (LENGTH - i - 1));
                        int digitChar = substring.chars().max().getAsInt();
                        startIndex += substring.indexOf(digitChar) + 1;
                        return digitChar;
                    }).boxed().map(i -> String.valueOf(i - '0'))
                    .collect(Collectors.joining())
                );
            })
            .sum());
}