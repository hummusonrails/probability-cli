# Bayesian Probability Calculator CLI

Welcome to the Bayesian Probability Calculator CLI! This command-line tool is designed to help you update your beliefs about the probability of an event based on new evidence. Using Bayesian probability, you can make better decisions under uncertainty.

## Overview

Bayesian probability is a powerful approach to reasoning under uncertainty. It allows you to combine your prior beliefs with new evidence to form updated beliefs, represented as posterior probabilities. The Bayesian Probability Calculator makes this process simple and interactive by guiding you through the input of prior probability, likelihood, and evidence.

## Getting Started

### Prerequisites

To run the Bayesian Probability Calculator, you need to have Rust installed on your machine. If you haven't already, you can install Rust by following the instructions on the official Rust website.

### Installation

Clone the repository:

```bash
git clone https://github.com/bencgreenberg/probability_calculator.git
```

Change to the project directory:

```bash
cd bayesian_probability_calculator
```

Build and run the project:

```bash
cargo run
```

## Usage

Upon running the Bayesian Probability Calculator, you'll be guided through a series of prompts. Follow the instructions and provide the requested information:

* Describe the event or scenario you're calculating the probability for.
* Provide the prior probability, which represents your initial belief about the probability of the event, based on your knowledge or experience.
* Provide the likelihood, which represents the probability of observing the evidence given that the event occurred.
* Provide the evidence, which represents the overall probability of observing the evidence.

The Bayesian Probability Calculator will then calculate the posterior probability, which is the updated probability of the event given the evidence. This result will be displayed as a percentage.

## Contributing

Contributions are welcome! If you'd like to improve the Bayesian Probability Calculator or suggest new features, please submit a pull request or create an issue. More information on contributing to the project can be found in the [contributing guidelines](CONTRIBUTING.md).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.