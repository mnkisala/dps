#[derive(Debug)]
pub enum CV<'a> {
    Const(f32),
    Buffer { buffer: &'a [f32], pos: usize },
}

impl<'a> CV<'a> {
    #[inline(always)]
    pub fn next_value(&mut self) {
        match self {
            CV::Const(_) => {}
            CV::Buffer { buffer, pos } => {
                *self = CV::Buffer {
                    buffer,
                    pos: *pos + 1,
                }
            }
        }
    }

    #[inline(always)]
    pub fn value(&self) -> f32 {
        match self {
            CV::Const(value) => *value,
            CV::Buffer { buffer, pos } => buffer[*pos],
        }
    }
}

impl From<f32> for CV<'_> {
    #[inline(always)]
    fn from(value: f32) -> Self {
        CV::Const(value)
    }
}

impl<'a> From<&'a [f32]> for CV<'a> {
    #[inline(always)]
    fn from(buffer: &'a [f32]) -> Self {
        CV::Buffer { buffer, pos: 0 }
    }
}

impl<'a> std::ops::Deref for CV<'a> {
    type Target = f32;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            CV::Const(value) => value,
            CV::Buffer { buffer, pos } => &buffer[*pos],
        }
    }
}
