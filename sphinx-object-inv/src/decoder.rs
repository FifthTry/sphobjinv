impl sphinx_object_inv::Data {
    // https://sphobjinv.readthedocs.io/en/latest/syntax.html
    pub fn from(
        r: &mut impl std::io::BufRead,
    ) -> sphinx_object_inv::Result<sphinx_object_inv::Data> {
        let mut buf = String::with_capacity(1024);

        // "The first line must be exactly"
        if r.read_line(&mut buf)? == 0 || buf != "# Sphinx inventory version 2\n" {
            eprintln!("buf: {:?}", buf);
            return Err(sphinx_object_inv::Error::InvalidHeader {
                message: "file does not start with # Sphinx inventory version 2".to_string(),
            });
        }

        Ok(sphinx_object_inv::Data {
            // "The second and third lines must obey the template"
            project: read_header("# Project", &mut buf, r)?,
            version: read_header("# Version", &mut buf, r)?,
            entries: read_entries(&mut buf, r)?,
        })
    }
}

fn read_entries(
    buf: &mut String,
    r: &mut impl std::io::BufRead,
) -> sphinx_object_inv::Result<Vec<sphinx_object_inv::Entry>> {
    use std::io::BufRead;

    buf.clear();

    // "The fourth line must contain the string zlib somewhere within it"
    if r.read_line(buf)? == 0 || !buf.contains("zlib") {
        eprintln!("buf: {:?}", buf);
        return Err(sphinx_object_inv::Error::InvalidHeader {
            message: "the fourth line does not contain zlib".to_string(),
        });
    }

    let z = std::io::BufReader::new(flate2::bufread::ZlibDecoder::new(r));
    let mut o = vec![];
    for line in z.lines() {
        o.push(read_entry(&line?)?);
    }

    Ok(o)
}

fn read_entry(s: &str) -> sphinx_object_inv::Result<sphinx_object_inv::Entry> {
    // https://github.com/bskinn/sphobjinv/blob/7d21f634/src/sphobjinv/re.py#L67
    static RE: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(|| {
        regex::Regex::new(
            r#"(?x)
            ^                               # Start of line
            (?P<name>.+?)                   # --> Name
            \s+                             # Dividing space
            (?P<domain>[^\s:]+)             # --> Domain
            :                               # Dividing colon
            (?P<role>[^\s:]+)               # --> Role
            \s+                             # Dividing space
            (?P<priority>-?\d+)             # --> Priority
            \s+?                            # Dividing space
            (?P<uri>\S*)                    # --> URI
            \s+                             # Dividing space
            (?P<dispname>.+?)               # --> Display name, lazy b/c possible CR
            \r?$                            # Ignore possible CR at EOL

        "#,
        )
        .unwrap()
    });

    let caps = match RE.captures(s) {
        Some(v) => v,
        None => {
            return Err(sphinx_object_inv::Error::InvalidHeader {
                message: format!("not in expected pattern: {}", s),
            });
        }
    };
    Ok(sphinx_object_inv::Entry {
        name: caps["name"].to_string(),
        domain: caps["domain"].to_string(),
        role: caps["role"].to_string(),
        uri: if caps["uri"].ends_with("$") {
            caps["uri"].replace("$", &caps["name"])
        } else {
            caps["uri"].to_string()
        },
        dispname: if &caps["dispname"] == "-" {
            caps["name"].to_string()
        } else {
            caps["dispname"].to_string()
        },
        priority: match caps["priority"].parse() {
            Ok(v) => v,
            Err(e) => {
                dbg!(caps);
                return Err(sphinx_object_inv::Error::InvalidHeader {
                    message: format!("priority is not an integer: {:?}", e),
                });
            }
        },
    })
}

fn read_header(
    name: &str,
    buf: &mut String,
    r: &mut impl std::io::BufRead,
) -> sphinx_object_inv::Result<String> {
    buf.clear();
    if r.read_line(buf)? == 0 || !buf.starts_with(name) {
        return Err(sphinx_object_inv::Error::InvalidHeader {
            message: format!("file does not contain {}", name),
        });
    }
    Ok(buf[name.len() + 1..].trim().to_string())
}
