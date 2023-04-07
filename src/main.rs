use colored::*;
use std::io;

/// The main function of the Bayesian Probability Calculator.
/// It prompts the user for input, performs the Bayesian calculation,
/// and prints the results.
fn main() {
    println!("Welcome to the Bayesian Probability Calculator!");
    println!();
    println!("Bayesian probability is a powerful approach to update your beliefs based on new evidence. It can help you make better decisions under uncertainty.");
    println!();

    println!("{}", "+-----------+".red());
    println!("{}", "| Prior     | ->".red());
    println!("{}", "| Probability|".red());
    println!("{}", "+-----------+".red());

    println!("{}", "    +-----------+".green());
    println!("{}", "    | Likelihood | ->".green());
    println!("{}", "    |  & Evidence |".green());
    println!("{}", "    +-----------+".green());

    println!("{}", "    +--------------+".blue());
    println!("{}", "    | Posterior    |".blue());
    println!("{}", "    | Probability  |".blue());
    println!("{}", "    +--------------+".blue());
    println!();

    println!("In the next steps, you'll be asked to provide a description of the event you're calculating, as well as three data points: prior probability, likelihood, and evidence.");
    println!("To answer the data points, you can use your own judgment, expert opinions, or available data. You can also search online for similar scenarios to help estimate probabilities."); 
    println!("Keep in mind that Bayesian reasoning is an iterative process, and you can update your probabilities as new evidence becomes available.");
    println!();
    println!();

    println!("{}", "Describe the thing being calculated:".yellow());
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");

    println!();
    println!();
    println!("Prior probability represents your initial belief about the probability of an event, before considering any new evidence.");
    println!("It is a percentage value between 0% (event is impossible) and 100% (event is certain), based on your knowledge or experience.");
    let prior = get_data_point("Enter the prior probability (in percentage, e.g., 50 or 50%):".yellow());

    println!();
    println!();
    println!("Likelihood represents how probable the new evidence is, assuming the event is true.");
    println!("It is a percentage value between 0% (evidence is impossible if the event is true) and 100% (evidence is certain if the event is true), based on how well the evidence supports the event.");
    let likelihood = get_data_point("Enter the likelihood (in percentage, e.g., 50 or 50%):".yellow());

    println!();
    println!();
    println!("Evidence represents the probability of observing the new evidence, taking into account all possible scenarios.");
    println!("It is a percentage value between 0% (evidence is impossible) and 100% (evidence is certain), based on how likely the evidence is in general.");
    let evidence = get_data_point("Enter the evidence (in percentage, e.g., 50 or 50%):".yellow());

    println!();
    println!();
    let posterior = bayesian(prior, likelihood, evidence);
    let percentage_posterior = posterior * 100.0;
    println!(
        "{}",
        format!(
            "Based on the information provided, the probability for '{}' is {:.2}%",
            description.trim(),
            percentage_posterior
        )
        .magenta()
    );
}

/// Prompts the user for input based on the provided prompt message.
/// Loops until the user provides a valid percentage input.
///
/// # Arguments
///
/// * `prompt` - A colored string representing the prompt message to display to the user.
///
/// # Returns
///
/// * A f64 value representing the parsed percentage as a decimal fraction.
fn get_data_point(prompt: ColoredString) -> f64 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if let Some(value) = get_percentage(&input) {
            return value;
        } else {
            println!("{}", "Invalid input: Please enter a valid percentage between 0% and 100%.".red());
        }
    }
}

/// Parses the user input as a percentage and converts it to a decimal fraction.
///
/// # Arguments
///
/// * `input` - A string slice representing the user input.
///
/// # Returns
///
/// * An Option<f64> value representing the parsed percentage as a decimal fraction.
///   Returns None if the input is invalid or out of range.
fn get_percentage(input: &str) -> Option<f64> {
    let value = input.trim().trim_end_matches('%').parse::<f64>().ok()?;
    if (0.0..=100.0).contains(&value) {
        Some(value / 100.0)
    } else {
        None
    }
}

/// Calculates the posterior probability using Bayesian reasoning.
///
/// # Arguments
///
/// * `prior` - The prior probability as a decimal fraction.
/// * `likelihood` - The likelihood as a decimal fraction.
/// * `evidence` - The evidence as a decimal fraction.
///
/// # Returns
///
/// * A f64 value representing the posterior probability as a decimal fraction.
fn bayesian(prior: f64, likelihood: f64, evidence: f64) -> f64 {
    (prior * likelihood) / evidence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_percentage_happy() {
        assert_eq!(get_percentage("50").unwrap(), 0.5);
        assert_eq!(get_percentage("25.5%").unwrap(), 0.255);
        assert_eq!(get_percentage("0%").unwrap(), 0.0);
        assert_eq!(get_percentage("100").unwrap(), 1.0);
    }

    #[test]
    fn test_get_percentage_unhappy() {
        assert!(get_percentage("101%").is_none());
        assert!(get_percentage("-1").is_none());
        assert!(get_percentage("abcd").is_none());
        assert!(get_percentage("12a3").is_none());
    }

    #[test]
    fn test_bayesian() {
        let prior = 0.5;
        let likelihood = 0.8;
        let evidence = 0.6;
        let expected_posterior = (prior * likelihood) / evidence;
        let calculated_posterior = bayesian(prior, likelihood, evidence);
        assert_eq!(calculated_posterior, expected_posterior);
    }

    #[test]
    fn test_bayesian_zero_prior() {
        let prior = 0.0;
        let likelihood = 0.8;
        let evidence = 0.6;
        let expected_posterior = 0.0;
        let calculated_posterior = bayesian(prior, likelihood, evidence);
        assert_eq!(calculated_posterior, expected_posterior);
    }

    #[test]
    fn test_bayesian_zero_likelihood() {
        let prior = 0.5;
        let likelihood = 0.0;
        let evidence = 0.6;
        let expected_posterior = 0.0;
        let calculated_posterior = bayesian(prior, likelihood, evidence);
        assert_eq!(calculated_posterior, expected_posterior);
    }

    #[test]
    fn test_bayesian_zero_evidence() {
        let prior = 0.5;
        let likelihood = 0.8;
        let evidence = 0.0;
        let calculated_posterior = bayesian(prior, likelihood, evidence);
        assert!(calculated_posterior.is_nan());
    }

    #[test]
    fn test_bayesian_full_certainty() {
        let prior = 1.0;
        let likelihood = 1.0;
        let evidence = 1.0;
        let expected_posterior = 1.0;
        let calculated_posterior = bayesian(prior, likelihood, evidence);
        assert_eq!(calculated_posterior, expected_posterior);
    }
}