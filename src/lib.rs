use std::io::{stdout, Write};
mod getch;

pub struct Interpreter<const N: usize = 65536> {
    code: [u8; N],
    c_ptr: usize,
    d_ptr: usize,
}

impl<const N: usize> Interpreter<N> {
    pub fn new<T>(program: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<u8>,
    {
        let mut code = [0; N];
        for (i, c) in program.into_iter().enumerate() {
            code[i + 1] = c.into();
        }
        Self {
            code,
            d_ptr: 1,
            c_ptr: 0,
        }
    }

    pub fn interpret(&mut self) {
        let mut loop_nest = Vec::new();
        let mut ptr_stack = Vec::new();
        let mut cell_stack = Vec::new();
        let getch = getch::Getch::new();
        let mut stdout = stdout();
        loop {
            let mut inc = true;
            let data = self.code[self.d_ptr];
            // println!("{} {} {} {} {} {}", self.c_ptr, self.code[self.c_ptr], self.code[self.c_ptr] as char, self.d_ptr, data, data as char);
            // if self.c_ptr > 500 { break;}
            match self.code[self.c_ptr] {
                b'+' => self.code[self.d_ptr] = data.wrapping_add(1),
                b'-' => self.code[self.d_ptr] = data.wrapping_sub(1),
                b'>' => self.d_ptr += 1,
                b'<' => self.d_ptr -= 1,
                b'.' => {
                    write!(stdout, "{}", data as char).unwrap();
                    stdout.flush().unwrap();
                }
                b',' => {
                    self.code[self.d_ptr] = getch.getch().unwrap();
                }
                b'[' => {
                    if data != 0 {
                        loop_nest.push(self.c_ptr);
                    } else {
                        let mut nesting = 1u32;
                        loop {
                            self.c_ptr += 1;
                            match self.code[self.c_ptr] {
                                b'[' => nesting += 1,
                                b']' => match nesting.checked_sub(1) {
                                    Some(0) => break,
                                    Some(k) => nesting = k,
                                    None => panic!(),
                                },
                                _ => (),
                            }
                        }
                    }
                }
                b']' => {
                    if let Some(k) = loop_nest.last() {
                        if data != 0 {
                            self.c_ptr = *k;
                        } else {
                            loop_nest.pop();
                        }
                    } else {
                        panic!()
                    }
                }
                b'$' => break,
                b'/' => self.d_ptr += 4,
                b'\\' => self.d_ptr -= 4,
                b'#' => {
                    self.c_ptr = self.d_ptr;
                    inc = false
                }
                b'%' => self.d_ptr = self.c_ptr,

                b'1' => self.d_ptr = data as usize,
                b'2' => self.c_ptr = data as usize,
                b'3' => self.d_ptr += data as usize,
                b'4' => self.c_ptr += data as usize,
                b'5' => self.code[self.d_ptr] = (self.c_ptr % 256) as u8,
                b'6' => self.code[self.d_ptr] = (self.d_ptr % 256) as u8,

                b'(' => {
                    while self.code[self.c_ptr] != b')' {
                        self.c_ptr += 1;
                    }
                }
                b')' => panic!(),
                b'&' => ptr_stack.push(self.d_ptr),
                b'*' => {
                    if let Some(k) = ptr_stack.pop() {
                        self.d_ptr = k;
                    } else {
                        panic!()
                    }
                }
                b'{' => cell_stack.push(data),
                b'}' => {
                    if let Some(k) = cell_stack.pop() {
                        self.code[self.d_ptr] = k;
                    } else {
                        panic!()
                    }
                }
                _ => (),
            }
            if inc {
                self.c_ptr += 1;
            }
        }
    }
}
