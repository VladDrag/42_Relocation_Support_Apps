/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   cli.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vdragomi <vdragomi@42student.wolfsburg.de> +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/01/03 12:18:21 by vdragomi          #+#    #+#             */
/*   Updated: 2022/01/05 15:25:09 by vdragomi         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::clone::Clone;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "kursuscal",
    about = "A 42 Wolfsburg core curriculum finishing time calculator;"
)]
pub enum Action {
    ///Shows the official time required to finish a project
    Time {
        #[structopt()]
        project: String,
    },
    ///Calls the calculator for the remaining time to finish the core, based on your indication of a project
    Calc {
        #[structopt()]
        calculator: String,
    },
}

#[derive(Debug, StructOpt)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
}
