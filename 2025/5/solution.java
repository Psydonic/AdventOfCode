import java.io.IOException;
import java.time.temporal.ValueRange;

final String INPUT_FILE = "./input.txt";
ValueRange currentRange;

void main() throws IOException {
    String[] file = Files.readString(Path.of(INPUT_FILE))
        .split(System.lineSeparator() + System.lineSeparator());
    
    // Parse ranges
    List<ValueRange> ranges = List.of(file[0].split(System.lineSeparator()))
        .stream()
        .map(line -> ValueRange.of(Long.valueOf(line.split("-")[0]), Long.valueOf(line.split("-")[1])))
        .sorted((a, b) -> Long.valueOf(a.getMinimum()).compareTo(Long.valueOf(b.getMinimum())))
        .toList();
        
    // Parse IDs and filter based on ranges
    long total = List.of(file[1].split(System.lineSeparator()))
            .stream()
            .map(line -> Long.valueOf(line))
            .filter(value -> ranges
                    .stream()
                    .takeWhile(r -> r.getMinimum() <= value)
                    .anyMatch(r -> value <= r.getMaximum()))
            .count();

    System.out.println(total);

    // Merge the ranges
    List<ValueRange> mergedRanges = new ArrayList<>();
    currentRange = ranges.get(0);
    ranges.stream().forEach(nextRange -> {
        if (nextRange.getMinimum() <= currentRange.getMaximum() + 1) {
            currentRange = ValueRange.of(currentRange.getMinimum(), 
                Math.max(currentRange.getMaximum(), nextRange.getMaximum()));
        } else {
            mergedRanges.add(currentRange);
            currentRange = nextRange;
        }
    });
    mergedRanges.add(currentRange);

    // Print the total range coverage
    System.out.println(mergedRanges
        .stream()
        .mapToLong(range -> range.getMaximum() - range.getMinimum() + 1)
        .sum());
}