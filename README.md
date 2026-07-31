## Statistics Calculator

A simple Rust program that calculates the median and mode of a list of integers.

#### Features

- Median: Finds the middle value of a sorted list
- Even-length lists: average of the two middle values
- Odd-length lists: the middle value
- Mode: Finds the most frequently occurring value

#### Usage

###### Prerequisites

- Rust (latest stable version)

###### Running the Program

```bash
# Clone the repository
git clone <repository-url>
cd <project-directory>

# Run the program
cargo run
```

###### Sample Output

```
List: [2, 5, 1, 8, 2, 9, 3, 2, 7] 
 Median: 3 
 Mode: 2
```

#### How It Works

###### Median Calculation

1. Creates a sorted copy of the list (original data remains unchanged)
2. For odd-length lists: returns the middle element
3. For even-length lists: returns the average of the two middle elements

###### Mode Calculation

1. Counts frequency of each number using a HashMap
2. Returns the number with the highest frequency

#### Code Structure

###### Function Description
- find_median(numbers: &Vec<i32>) -> f64 Calculates median as a floating-point value
- find_mode(numbers: &Vec<i32>) -> i32 Calculates mode as an integer

#### Limitations

- Only works with i32 integers
- Returns only one mode (if multiple modes exist, returns the first one found)
- Does not handle empty vectors (would panic)


#### License

MIT License
