/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vdragomi <vdragomi@42student.wolfsburg.de> +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/01/03 12:18:42 by vdragomi          #+#    #+#             */
/*   Updated: 2022/01/03 14:46:57 by vdragomi         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod cli;
use cli::{Action::*, CommandLineArgs};
use structopt::StructOpt;

struct Project {
    name: String,
    duration: u16,
}

impl Project {
    fn new(name: String, duration: u16) -> Project {
        Project {
            name: name,
            duration: duration,
        }
    }
}

fn main() {
    cli::CommandLineArgs::from_args();
    let libft = Project::new(String::from("Libft"), 70);
    let CommandLineArgs { action } = cli::CommandLineArgs::from_args();

    match action {
        Time { project } => {
            if project.to_lowercase().trim() == libft.name.to_lowercase() {
                println!("The duration of {} is {}.", libft.name, libft.duration);
            } else {
                println!("The input given is not linked to any existing project.");
            }
        }
        Calc { calculator: _ } => {
            println!("{}", libft.name)
        }
    }
}
