use error_chain::error_chain;
use url::form_urlencoded::{byte_serialize, parse};
use url::{Host, Origin, ParseError, Position, Url};

error_chain! {
    foreign_links{
        UrlParse(url::ParseError);
    }
    errors{
        CannotBeABase
    }
}

fn main() -> Result<()> {
    println!("将字符串编码为 application/x-www-form-urlencoded 表单语法!");

    let urlencoded: String = byte_serialize("What is ❤?".as_bytes()).collect();
    println!("urlencoded: {}", urlencoded);
    assert_eq!(urlencoded, "What+is+%E2%9D%A4%3F");

    let decoded: String = parse(urlencoded.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect();
    println!("decoded:'{}'", decoded);
    assert_eq!(decoded, "What is ❤?");

    println!("解析 URL 字符串为 Url 类型=========start=======");
    let s = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";
    let parsed = Url::parse(&s).unwrap();
    println!("The path part of the URL is: {}", parsed.path());
    println!("{:#?}", parsed);
    println!("解析 URL 字符串为 Url 类型========= end =======");

    println!("\n通过移除路径段创建基本 URL=======start======");
    let full = "https://github.com/rust-lang/cargo?asdf";

    let url = Url::parse(full)?;
    let base = base_url(url)?;

    assert_eq!(base.as_str(), "https://github.com/");
    println!("The base of the URL is: {}", base);
    println!("通过移除路径段创建基本 URL======= end  ======\n");

    println!("\n从基本 URL 创建新 URLs =======start======");
    let path = "/rust-lang/cargo";
    let gh = build_github_url(path)?;
    assert_eq!(gh.as_str(), "https://github.com/rust-lang/cargo");
    println!("The joined URL is: {}", gh);
    println!("从基本 URL 创建新 URLs ======= end ======\n");

    println!("\n 提取 URL 源（scheme / host / port） ========start====");
    let s = "ftp://rust-lang.org/examples";
    let url = Url::parse(s)?;

    assert_eq!(url.scheme(), "ftp");
    assert_eq!(url.host(), Some(Host::Domain("rust-lang.org")));
    assert_eq!(url.port_or_known_default(), Some(21));
    println!("The origin is as expected!下面origin 方法产生相同的结果。");
    let s = "ftp://rust-lang.org/examples";
    let url = Url::parse(s)?;

    let expected_scheme = "ftp".to_owned();
    let expected_host = Host::Domain("rust-lang.org".to_owned());
    let expected_port = 21;
    let expected = Origin::Tuple(expected_scheme, expected_host, expected_port);
    let origin = url.origin();
    assert_eq!(origin, expected);
    println!("The origin is as expected!");
    println!(" 提取 URL 源（scheme / host / port） =======end====");

    let parsed = Url::parse("https://github.com/rust-lang/rust/issues?labels=E-easy&state=open")?;
    let cleaned=&parsed[..Position::AfterPath];  //从 URL 移除片段标识符和查询对
    // let cleaned: &str = &parsed[..Position::AfterPath];
    println!("cleaned: {}", cleaned);
    Ok(())
}

fn build_github_url(path: &str) -> Result<Url> {
    const GITHUB: &'static str = "https://github.com/";

    let base = Url::parse(GITHUB).expect("Invalid");
    let joined = base.join(path)?;
    Ok(joined)
}

fn base_url(mut url: Url) -> Result<Url> {
    match url.path_segments_mut() {
        Ok(mut path) => {
            path.clear();
        }
        Err(_) => {
            return Err(Error::from_kind(ErrorKind::CannotBeABase));
        }
    }
    url.set_query(None);
    Ok(url)
}
