use std::fs::File;
use std::io::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// input file path
    #[structopt(name = "file", default_value = "entry.tex")]
    fpath: String,

    /// output file path
    #[structopt(short = "o", long = "out")]
    out: Option<String>,
}

pub fn get_file_contents(fpath: &str) -> String {
    let mut f = File::open(fpath).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

fn main() {
    let opt = Opt::from_args();
    let contents = get_file_contents(&opt.fpath);
    let template_html = get_file_contents("template.html");

    let re_titles =
        regex::Regex::new(r"[\r\n|\n|\r]#\s(?P<title>[^\r\n|\n|\r]+)[\r\n|\n|\r]").unwrap();
    let mut contents_html = format!(
        "<div>{}</div>",
        re_titles.replace_all(
            &format!("\n{}", &contents),
            r#"</div>
<div id="${title}">
<h2>${title}</h2>
"#,
        )
    );
    let re_breaks = regex::Regex::new(r"[\r\n|\n|\r]{2}").unwrap();
    contents_html = re_breaks
        .replace_all(&contents_html, "\n<p></p>\n")
        .to_string();

    let re_links = regex::Regex::new(r"#(?P<title>\S+)\s").unwrap();
    contents_html = re_links
        .replace_all(&contents_html, r##"<a href="#${title}">${title}</a>"##)
        .to_string();

    let result = template_html.replace("<template></template>", &contents_html);

    let mut file = File::create("result.html").unwrap();
    writeln!(&mut file, "{}", &result).unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let re =
            regex::Regex::new(r"[\r\n|\n|\r]#\s(?P<title>[^\r\n|\n|\r]+)[\r\n|\n|\r]").unwrap();
        let text = r#"
# Hello, world!
        "#;
        for caps in re.captures_iter(text) {
            let title = format!("{}", &caps["title"]);
            assert_eq!(&title, "Hello, world!");
        }
    }
}
