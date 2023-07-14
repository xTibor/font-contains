// font-contains őŐűŰ | xargs -d "\n" basename -a | sort

fn main() {
    let input_characters = std::env::args().skip(1).next().unwrap();

    let home_dir = home::home_dir().unwrap().display().to_string();
    let glob_pattern = format!("{}/.fonts/**/*.{{ttf,otf}}", home_dir);

    for font_path in globwalk::glob(glob_pattern)
        .unwrap()
        .filter_map(Result::ok)
        .map(|dir_entry| dir_entry.path().to_owned())
        .filter(|path_buf| path_buf.is_file())
    {
        let data = std::fs::read(&font_path).unwrap();
        let face = ttf_parser::Face::parse(&data, 0).unwrap();

        if input_characters.chars().all(|c| face.glyph_index(c).is_some()) {
            println!("{}", font_path.display());
        }
    }
}
