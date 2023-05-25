use libc::{c_char, c_double, c_int, fclose, fgets, fopen, fscanf, EOF, FILE};
use std::ffi::CString;

struct File {
    stream: *mut FILE,
}

/* At this place _starter_ code contains `to_c_string(string: &str) -> Vec<i8>` function, which I replaced in spite
of `ffi::CString` methods during debugging, and didn't want come from this standard solution. */

impl File {
    fn open(_path: &str) -> Option<Self> {
        let path_c = CString::new(_path).expect("it's exercise --- `_path` should be there");
        let mode = CString::new("r").unwrap();
        let result = unsafe { fopen(path_c.as_ptr(), mode.as_ptr()) };
        if result.is_null() {
            None
        } else {
            Some(File { stream: result })
        }
    }

    /// Returns `None` if file couldn't been read (to `String`) OR it
    /// doesn't contain EOL.
    fn read_string(&mut self) -> Option<String> {
        let mut buffer_current = [i8::default(); 512];
        // looks like it's ok to get just long enough start of the line; let it be 512 symbols
        let success = unsafe {
            fgets(
                buffer_current.as_mut_ptr() as *mut c_char,
                // note two bytes for EOL symbols in `buf`
                510,
                self.stream,
            )
        };
        if success.is_null() {
            return None;
        }
        Some(
            buffer_current
                .into_iter()
                .map(|x| x.to_ne_bytes()[0] as u8)
                .skip_while(|x| x == &0)
                .map(|x| x as char)
                .collect(),
        )
    }

    fn read_i64(&mut self) -> Option<i64> {
        let mut result: c_int = Default::default();
        let mode = CString::new("%d").unwrap();
        match unsafe { fscanf(self.stream, mode.as_ptr(), &mut result) } {
            EOF | 0 => None,
            _ => Some(result.into()),
        }
    }

    fn read_f64(&mut self) -> Option<f64> {
        let mut result: c_double = Default::default();
        let mode = CString::new("%lf").unwrap();
        match unsafe { fscanf(self.stream, mode.as_ptr(), &mut result) } {
            EOF | 0 => None,
            _ => Some(result.into()),
        }
    }

    fn read_char(&mut self) -> Option<char> {
        let mut result: c_char = Default::default();
        let mode = CString::new(" %c").unwrap();
        match unsafe { fscanf(self.stream, mode.as_ptr(), &mut result) } {
            EOF | 0 => None,
            _ => Some(result.to_be_bytes()[0].into()),
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        println!("Dropping file.");
        let success = unsafe { fclose(self.stream) };
        if success != 0 {
            panic!("...")
        }
    }
}

fn main() {
    let mut file = File::open("data/test_file.txt").expect("Could not open file.");
    let s = file.read_string().unwrap();
    let i = file.read_i64().unwrap();
    let f = file.read_f64().unwrap();
    let c = file.read_char().unwrap();

    println!("{s} {i} {f} {c}");
}
