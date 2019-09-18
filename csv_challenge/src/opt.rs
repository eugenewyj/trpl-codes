use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(name = "csv_challenge", about = "用法")]
pub struct Opt {
    #[structopt(help = "输入文件")]
    pub input: String,
    #[structopt(help = "列名")]
    pub column_name: String,
    #[structopt(help = "替换后列内容")]
    pub replacement: String,
    #[structopt(help = "输出文件，默认为标准输出")]
    pub output: Option<String>,
}