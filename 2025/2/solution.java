import java.io.IOException;
import java.util.stream.LongStream;

final String INPUT_FILE = "./input.txt";

void main() throws IOException {
    System.out.println(
        List.of(Files.readString(Path.of(INPUT_FILE))
            .split(","))
            .stream()
            .mapToLong((range) -> {
                long start = Long.valueOf(range.split("-")[0]);
                long end = Long.valueOf(range.split("-")[1]);
                return LongStream.rangeClosed(start, end)
                    .filter((number) -> String.valueOf(number)
                        .matches("^(.+?)\\1+$"))
                    .sum();
            }).sum());
}