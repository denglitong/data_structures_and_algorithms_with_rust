use std::rc::Rc;

#[derive(Debug)]
struct FileName {
    name: Rc<String>,
    ext: Rc<String>,
}

#[cfg(test)]
mod tests {
    use crate::macros::FileName;
    use std::rc::Rc;

    #[test]
    fn test1() {
        let filename = FileName {
            name: Rc::new("/tmp/not/a/file".to_string()),
            ext: Rc::new("file".to_string()),
        };
        println!("{:?}", filename);
    }
}
