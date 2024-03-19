# HDI Country Classifier for School Project

This Rust program reads data from a text file containing Human Development Index (HDI) values for various countries and classifies them into different categories based on their HDI scores. It then randomly selects a sample of countries from each category and prints them out.

## Prerequisites

Before running this program, ensure you have the following installed on your system:

- [Rust](https://www.rust-lang.org/tools/install): The Rust programming language and its tools.
- [Git](https://git-scm.com/downloads): Version control system for managing the source code.

## Getting Started

To get started with this program, follow these steps:

1. **Clone the Repository**: Open your terminal and clone this repository using the following command:

   ```bash
   git clone https://github.com/your_username/hdi_country_classifier.git
   ```

2. **Navigate to the Project Directory**: Change your current directory to the cloned repository:

   ```bash
   cd hdi_country_classifier
   ```

3. **Build and Run the Program**: Build and run the program using Cargo, the Rust package manager:

   ```bash
   cargo run
   ```

   This command will compile the code and execute the program.

## Usage

After running the program, you will see the sample countries printed for each HDI category:

- Very high human development
- High human development
- Medium human development
- Low human development

## Input Data

The program expects a text file named `HDR21-22_Statistical_Annex_HDI_Table.txt` located in the `data` directory. This file should contain country names and their corresponding HDI scores separated by spaces. This file comes included with the repository.

## Output

The program outputs a sample of countries from each HDI category, randomly selected from the input data.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

This program utilizes the [rand](https://crates.io/crates/rand) crate for random number generation.
