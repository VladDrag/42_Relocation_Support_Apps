/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vdragomi <vdragomi@42student.wolfsburg.de> +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/01/03 12:18:42 by vdragomi          #+#    #+#             */
/*   Updated: 2022/01/05 16:30:51 by vdragomi         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod cli;
use cli::{Action::*, CommandLineArgs};
use structopt::StructOpt;

#[derive(Clone, Debug)]
struct Project<'a> {
    name: &'a str,
    duration: u16,
}

impl Project<'_> {
    fn new(name: &str, duration: u16) -> Project {
        Project {
            name: name,
            duration: duration,
        }
    }
}

pub fn dup<Project: Clone>(vect: &[Project]) -> Vec<Project> {
    let vec = vect.to_vec();
    vec
}
// pub fn duration_left(projects: [Project; 10], start: usize) -> i32
// {
// 	for proj in projects[start..]
// 	{
// 		println!("{}", proj);
// 	}
// }

fn main() {
    cli::CommandLineArgs::from_args();
    let CommandLineArgs { action } = cli::CommandLineArgs::from_args();
    let projects = vec![
        Project::new("Libft", 70),
        Project::new("Get_next_line", 80),
        Project::new("Ft_Printf", 90),
        Project::new("Born2BeRoot", 100),
    ];
    let len = projects.len();
    let mut counter = 0;
    let mut sum: u32 = 0;
    println!(".........{}........", projects[1].name);

    for proj in dup(&projects) {
        counter += 1;
        match &action {
            Time { project } => {
                if project.to_lowercase().trim() == proj.name.to_lowercase() {
                    println!("The duration of {} is {} hours.", proj.name, proj.duration);
                    return;
                } else {
                }
            }
            Calc { calculator } => {
                if calculator.to_lowercase().trim() == proj.name.to_lowercase() {
                    for i in counter - 1..len {
                        sum += projects[i].duration as u32;
                    }
                    println!("The remaining time is aproximately {} hours, with a possible variation of 100-140 hours, depending on the curriculum circle that you are situated on", sum);
                    return;
                } else {
                }
            }
        }
    }
    println!("The input given is not linked to any project. Please retry.")
}
