use std::{cmp::max, collections::HashMap};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Name(String);

#[derive(Deserialize, Debug)]
pub struct Classname(String);

#[derive(Deserialize, Debug)]
pub struct GithubRepo(String);

#[derive(Deserialize, Debug)]
pub struct HeaderSource(String);

#[derive(Deserialize, Debug)]
pub struct Description(String);

#[derive(Deserialize, Debug)]
pub struct ProductPage(String);

pub struct TableColumn {
    data: Vec<String>,
}

impl TableColumn {
    pub fn get_longest_string_len(&self) -> Option<usize> {
        self.data.iter().map(String::len).max()
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Index(usize, usize);

#[derive(Default)]
pub struct Spreadsheet {
    max_row_idx: usize,
    max_col_idx: usize,
    data: HashMap<Index, String>,
}

impl Spreadsheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_bounds(&mut self, index: Index) {
        self.max_row_idx = max(self.max_row_idx, index.0);
        self.max_col_idx = max(self.max_col_idx, index.1);
    }

    pub fn set(&mut self, index: Index, value: String) {
        self.update_bounds(index);
        self.data.insert(index, value);
    }

    fn draw_table_hline(
        left_edge_joiner: &str,
        cell_join: &str,
        right_edge_joiner: &str,
        column_widths: &[usize],
    ) {
        // let left_edge_joiner = "┣";
        // let right_edge_joiner = "┫";
        let horizontal_line = "━";

        print!("{}", left_edge_joiner);
        for (i, column_width) in column_widths.iter().enumerate() {
            print!(
                "{}{}{}",
                horizontal_line,
                horizontal_line.repeat(*column_width),
                horizontal_line
            );
            if i != column_widths.len() - 1 {
                print!("{}", cell_join);
            } else {
                println!("{}", right_edge_joiner)
            }
        }
    }

    pub fn draw(&self) {
        let mut column_widths = vec![0; self.max_col_idx + 1];
        for row in 0..=self.max_row_idx {
            for col in 0..=self.max_col_idx {
                if let Some(cell_width) = self.data.get(&Index(row, col)).map(String::len) {
                    if cell_width >= column_widths[col] {
                        column_widths[col] = cell_width;
                    }
                }
            }
        }

        let top_left_corner = "┏";
        let top_right_corner = "┓";
        let top_edge_column_joiner = "┳";
        let vertical_separator = "┃";
        let cell_joiner = "╋";
        let left_edge_joiner = "┣";
        let right_edge_joiner = "┫";
        let bottom_left = "┗";
        let bottom_right = "┛";
        let bottom_cell_joiner = "┻";

        // print!("{}", top_left_corner);
        // for (i, column_width) in column_widths.iter().enumerate() {
        //     print!("{}{}{}", horizontal_line, horizontal_line.repeat(*column_width), horizontal_line);
        //     if i != column_widths.len() - 1 {
        //         print!("{}", top_edge_column_joiner);
        //     } else {
        //         println!("{}", top_right_corner)
        //     }
        // }

        Self::draw_table_hline(
            top_left_corner,
            top_edge_column_joiner,
            top_right_corner,
            &column_widths,
        );

        for row in 0..=self.max_row_idx {
            print!("{}", vertical_separator);
            for col in 0..=self.max_col_idx {
                let cell_data = self.data.get(&Index(row, col)).cloned().unwrap_or_default();
                let cell_data_len = cell_data.len();
                let cell_data = cell_data + &" ".repeat(column_widths[col] - cell_data_len);
                print!(" {} ", cell_data);
                print!("{}", vertical_separator);
            }
            println!();

            if row != self.max_row_idx {
                Self::draw_table_hline(
                    left_edge_joiner,
                    cell_joiner,
                    right_edge_joiner,
                    &column_widths,
                );
            } else {
                Self::draw_table_hline(
                    bottom_left,
                    bottom_cell_joiner,
                    bottom_right,
                    &column_widths,
                );
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct XChip {
    name: Name,
    classname: Classname,
    github_repository: GithubRepo,
    header_src: HeaderSource,
    description: Description,
    product_page: ProductPage,
}

impl XChip {
    pub fn get_pairs(&self) -> [(&str, &str); 6] {
        [
            ("Name", &self.name.0),
            ("Description", &self.description.0),
            ("GitHub Repo", &self.github_repository.0),
            ("Header Source", &self.header_src.0),
            ("Product Page", &self.product_page.0),
            ("C++ Class Name", &self.classname.0),
        ]
    }
    pub fn into_spreadsheet(&self) -> Spreadsheet {
        let mut spreadsheet = Spreadsheet::new();

        for (i, (key, value)) in (&self.get_pairs()).iter().enumerate() {
            spreadsheet.set(Index(i, 0), key.to_string());
            spreadsheet.set(Index(i, 1), value.to_string());
        }
        spreadsheet
    }
}
#[derive(Deserialize, Debug)]
pub struct XChipRegistry {
    xchips: Vec<XChip>,
}

pub fn list_chips(chips: &[XChip]) { 
    let mut spreadsheet = Spreadsheet::new();
    spreadsheet.set(Index(0, 0), String::from("Chip Name"));
    spreadsheet.set(Index(0, 1), String::from("Description"));
    for (i, chip) in chips.iter().enumerate() { 
        spreadsheet.set(Index(i + 1, 0), chip.name.0.clone());
        spreadsheet.set(Index(i + 1, 1), chip.description.0.clone());
    }
    spreadsheet.draw();
}
pub fn lib_main() {
    let arg_matches = clap::App::new("X-In-A-Box Chip Reference")
        .author("Andy Day")
        .about("A quick way to see xinabox stats")
        .arg(
            clap::Arg::with_name("list")
                .long("list")
                .help("Lists the (currently registered) xinabox chips")
                .takes_value(false)
                .conflicts_with("chip"),
        )
        .arg(
            clap::Arg::with_name("chip")
                .short("c")
                .long("chip")
                .value_name("CHIP_NAME")
                .help("lists data about the given chip")
                .conflicts_with("list")
                .takes_value(true),
        )
        .subcommand(clap::SubCommand::with_name("list")
                                      .about("lists the various chips"))
        .get_matches();

    let chips: Vec<XChip> = serde_json::from_str(include_str!("../chips.json")).unwrap();

    if let Some(chip_name) = arg_matches.value_of("chip") {
        let chip_name = chip_name.to_ascii_lowercase();
        let chip = chips.iter().find(|chip| chip.name.0.to_ascii_lowercase() == chip_name);
        
        match chip {
            Some(chip) => { 
                chip.into_spreadsheet().draw();
            }
            None => {
                eprintln!("No entry for chip name `{}`, exiting...", chip_name);
            }
        }
    }
    else if arg_matches.is_present("list") { 
        list_chips(&chips);
    } else if arg_matches.subcommand_matches("list").is_some() { 
        list_chips(&chips);
    } else { 
        list_chips(&chips);
    }
}
