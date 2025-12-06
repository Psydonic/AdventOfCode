import java.io.IOException;

final String INPUT_FILE = "./input.txt";
int total = 50;
int count = 0;

void main() throws IOException {    
    
    Files.lines(Path.of(INPUT_FILE))
        .forEach((line) -> {
            int number = Integer.valueOf(line.substring(1));    // Extract the number
            count += (number / 100);                            // for every 100 we know it must go past 0 once
            number %= 100;                                      // Find the remainder
            int newTotal = total + (line.charAt(0) == 'L' ? -number : number);  // Rotate the dial
            count += total != 0 && (newTotal <= 0 || newTotal >= 100) ? 1 : 0;  // Check if it went past zero
            total = Math.floorMod(newTotal, 100);                               // Assign the new total
        });
    
    System.out.println(count);
}