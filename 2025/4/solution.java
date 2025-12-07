import java.io.IOException;

final String INPUT_FILE= "./input.txt";

void main() throws IOException {
    Boolean[][] grid = Files.lines(Path.of(INPUT_FILE))
        .map(line -> line.chars().boxed().map(c -> c == '@').toArray(Boolean[]::new))
        .toArray(Boolean[][]::new);

    int total = 0;
    int localTotal;
    do {
        localTotal = 0;
        for (int y = 0; y < grid.length; y++) {
            for (int x = 0; x < grid[0].length; x++) {
                int surroundings = 0;
                for (int i = -1; i < 2; i++) {
                    for (int j = -1; j < 2; j++) {
                        if ((i == 0 && j == 0) || 
                            (x + i < 0) ||
                            (y + j < 0) ||
                            (x + i > grid[0].length - 1) ||
                            (y + j > grid.length - 1))
                            continue;
                        if (grid[y + j][x + i])
                            surroundings++;
                    }
                }

                if (surroundings < 4 && grid[y][x]) {
                    localTotal++;
                    grid[y][x] = false;
                }
            }
        } 
        total += localTotal;
    } while (localTotal > 0);
    
    System.out.println(total);
}