use clap::Parser;
use colored::*;
use crate::info::{packages::get_packages_info, php::get_php_info, project::get_project_info, InfoItem};

#[derive(Parser)]
#[clap(about = "Display information about the current project.")]
pub struct InfoCommand;

pub fn info(cmd: &InfoCommand) {
    let php = get_php_info();
    let project = get_project_info();
    let packages = get_packages_info();
    
    println!("{}", php.get_name().bold().green());

    let items = php.get_items();
    let longest_name_length = get_longest_name_length(items);

    for item in items {
        println!("{:<width$}    {}", item.get_name().bold(), item.get_value(), width = longest_name_length);
    }

    println!();
    println!("{}", project.get_name().bold().green());

    let items = project.get_items();
    let longest_name_length = get_longest_name_length(items);

    for item in items {
        println!("{:<width$}    {}", item.get_name().bold(), item.get_value(), width = longest_name_length);
    }

    println!();
    println!("{}", packages.get_name().bold().green());

    let items = packages.get_items();
    let longest_name_length = get_longest_name_length(items);

    for item in items {
        println!("{:<width$}    {}", item.get_name().bold(), item.get_value(), width = longest_name_length);
    }
}

fn get_longest_name_length(items: &[InfoItem]) -> usize {
    items.iter().map(|item| item.get_name().len()).max().unwrap_or(0)
}